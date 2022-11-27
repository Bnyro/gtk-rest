use std::cell::RefCell;
use std::rc::Rc;

use adw::subclass::prelude::AdwApplicationWindowImpl;
use adw::subclass::prelude::WidgetClassSubclassExt;
use glib::subclass::InitializingObject;
use gtk::gio::SimpleAction;
use gtk::glib::clone;
use gtk::glib::MainContext;
use gtk::glib::PRIORITY_DEFAULT;
use gtk::subclass::prelude::ObjectImplExt;
use gtk::subclass::prelude::ObjectSubclassExt;
use gtk::subclass::prelude::{ApplicationWindowImpl, ObjectImpl, ObjectSubclass};
use gtk::subclass::widget::{CompositeTemplateCallbacksClass, CompositeTemplateClass, WidgetImpl};
use gtk::subclass::window::WindowImpl;
use gtk::CheckButton;
use gtk::StringList;
use gtk::{glib, CompositeTemplate, DropDown, TemplateChild};
use gtk::{prelude::*, Box, Button, Entry, HeaderBar};
use sourceview5::traits::BufferExt;

use crate::client::Request;
use crate::kvpair::KvPair;
use crate::preferences;
use crate::preferences::utils::get_prefs;
use crate::preferences::KeyValuePair;
use crate::utils::format_json_string;

// ANCHOR: object
// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/bnyro/rest/window.ui")]
pub struct Window {
    #[template_child]
    pub headerbar: TemplateChild<HeaderBar>,
    #[template_child]
    pub workspaces: TemplateChild<DropDown>,
    #[template_child]
    pub workspaces_model: TemplateChild<StringList>,
    #[template_child]
    pub new_workspace_name: TemplateChild<Entry>,
    #[template_child]
    pub create_workspace: TemplateChild<Button>,
    #[template_child]
    pub requests: TemplateChild<DropDown>,
    #[template_child]
    pub requests_model: TemplateChild<StringList>,
    #[template_child]
    pub new_request_name: TemplateChild<Entry>,
    #[template_child]
    pub create_request: TemplateChild<Button>,
    #[template_child]
    pub url: TemplateChild<Entry>,
    #[template_child]
    pub send: TemplateChild<Button>,
    #[template_child]
    pub method: TemplateChild<DropDown>,
    #[template_child]
    pub isjsonbody: TemplateChild<CheckButton>,
    #[template_child]
    pub body: TemplateChild<Entry>,
    #[template_child]
    pub format_body: TemplateChild<Button>,
    #[template_child]
    pub headers: TemplateChild<Box>,
    #[template_child]
    pub add_header: TemplateChild<Button>,
    #[template_child]
    pub queries: TemplateChild<Box>,
    #[template_child]
    pub add_query: TemplateChild<Button>,
    #[template_child]
    pub response: TemplateChild<sourceview5::View>,
    pub header_pairs: RefCell<Vec<KeyValuePair>>,
    pub query_pairs: RefCell<Vec<KeyValuePair>>,
}
// ANCHOR_END: object

// ANCHOR: subclass
// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "GtkRestWindow";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}
// ANCHOR_END: subclass

// ANCHOR: template_callbacks
#[gtk::template_callbacks]
impl Window {
    #[template_callback]
    fn handle_send(&self, _button: &gtk::Button) {
        let request = Request::new(
            self.url.text().to_string(),
            self.body.text().to_string(),
            self.method.selected(),
            self.header_pairs.clone().take(),
            self.query_pairs.clone().take(),
            self.isjsonbody.is_active(),
        );

        let (sender, receiver) = MainContext::channel::<(String, Option<String>)>(PRIORITY_DEFAULT);
        let main_context = MainContext::default();

        // The long running operation runs now in a separate thread
        main_context.spawn_local(clone!(@strong sender => async move {
            // Deactivate the button until the operation is done

            let response = request.execute().await;

            if let Ok(response) = response {
                let headers = response.headers().clone();
                        let content_type = headers.get("Content-Type");
                        let ct_split: Vec<&str> =
                            content_type.unwrap().to_str().unwrap().split(";").collect();
                        let text = response.text().unwrap();
                        let ct = ct_split[0];
                sender.send((text, Some(ct.to_string()))).expect("Error sending data");
            }
        }));
        receiver.attach(
            None,
            clone!(@weak self as win => @default-return Continue(false),
                move |(text, header)| {
                    win.set_response_text(text, header);
                    Continue(true)
                }
            ),
        );
    }
}

impl Window {
    pub fn set_response_text(&self, text: String, content_type: Option<String>) {
        let buffer = sourceview5::Buffer::new(None);
        buffer.set_highlight_syntax(true);
        buffer.set_highlight_matching_brackets(true);
        buffer.set_text(text.as_str());

        if let Some(content_type) = content_type {
            let language = sourceview5::LanguageManager::default()
                .guess_language(None::<String>, Some(content_type.as_str()));

            buffer.set_language(language.as_ref());

            if content_type.contains("json") {
                buffer.set_text(&format_json_string(text));
            }
        }

        if let Some(ref scheme) = sourceview5::StyleSchemeManager::new().scheme("classic-dark") {
            buffer.set_style_scheme(Some(scheme));
        }

        self.response.set_buffer(Some(&buffer));
    }

    pub fn create_header(&self, pair: KeyValuePair) {
        let index = self.header_pairs.borrow().len();
        self.header_pairs.borrow_mut().push(pair);
        let on_change = clone!(@weak self as win => move |index, kvp| {
            win.header_pairs.borrow_mut()[index] = kvp;
        });
        let mut kv_pair = KvPair::new(index);
        let child = kv_pair.build(&self.headers, Rc::new(on_change));
        self.headers.append(&child);
    }

    pub fn create_query(&self, pair: KeyValuePair) {
        let index = self.query_pairs.borrow().len();
        self.query_pairs.borrow_mut().push(pair);
        let on_change = clone!(@weak self as win => move |index, kvp| {
            win.query_pairs.borrow_mut()[index] = kvp;
        });
        let mut kv_pair = KvPair::new(index);
        let child = kv_pair.build(&self.queries, Rc::new(on_change));
        self.queries.append(&child);
    }

    pub fn add_workspace(&self, workspace_name: String) {
        let size = self.workspaces_model.n_items();
        for i in 0..size {
            let str = self.workspaces_model.string(i);
            if str.is_some() && str.unwrap() == workspace_name {
                return;
            }
        }
        let mut prefs = crate::preferences::utils::get_prefs();
        self.workspaces_model.append(workspace_name.as_str());
        let mut workspace = preferences::Workspace::default();
        workspace.name = workspace_name;
        let mut request = preferences::Request::default();
        request.name = String::from("Default");
        workspace.requests.push(request);

        // save to the settings
        prefs.workspaces.push(workspace.clone());
        crate::preferences::utils::save_prefs(&prefs);

        // select the newly created workspace
        self.workspaces
            .set_selected(self.workspaces_model.n_items() - 1);

        // load the new workspace
        self.load_workspace(&workspace);
    }

    pub fn add_request(&self, request_name: String) {
        // check whether the request name already exists
        let size = self.requests_model.n_items();
        for i in 0..size {
            let str = self.requests_model.string(i);
            if str.is_some() && str.unwrap() == request_name {
                return;
            }
        }

        // create the new request
        self.requests_model.append(request_name.as_str());
        let mut request = preferences::Request::default();
        request.name = request_name;

        // load and save the new created request
        self.load_request(&request);

        // select the newly created item in the requests list
        self.requests
            .set_selected(self.requests_model.n_items() - 1);

        self.save_request();
    }

    pub fn load_request(&self, request: &preferences::Request) {
        self.save_request();

        self.body.set_text(request.body.as_str());

        self.url.set_text(request.target_url.as_str());
        self.method.set_selected(request.method);

        // remove all headers in the UI
        while let Some(child) = self.headers.first_child() {
            self.headers.remove(&child);
        }
        for i in 0..request.headers.len() {
            self.create_header(request.headers[i].clone());
        }

        // remove all queries in the UI
        while let Some(child) = self.queries.first_child() {
            self.queries.remove(&child);
        }
        for i in 0..request.queries.len() {
            self.create_query(request.queries[i].clone());
        }
    }

    pub fn load_workspace(&self, workspace: &preferences::Workspace) {
        println!("load workspace {:?}", workspace);

        self.requests_model
            .splice(0, self.requests_model.n_items(), &[]);

        for i in 0..workspace.requests.len() {
            self.requests_model
                .append(workspace.requests[i].name.as_str());
        }

        self.load_request(&workspace.requests[0]);
    }

    pub fn get_request(&self) -> preferences::Request {
        let mut request = preferences::Request::default();
        request.headers = self.header_pairs.clone().take();
        request.queries = self.query_pairs.clone().take();
        request.body = self.body.text().clone().to_string();
        request.target_url = self.url.text().to_string();
        let request_name = self.requests_model.string(self.requests.selected());
        if request_name.is_some() {
            request.name = request_name.unwrap().to_string();
        }
        request.method = self.method.selected();
        request
    }

    pub fn save_request(&self) {
        let mut prefs = crate::preferences::utils::get_prefs();

        let current_workspace = self.workspaces.selected() as usize;

        let request = self.get_request();

        let current_request_index = self.requests.selected() as usize;

        if current_request_index >= prefs.workspaces[current_workspace].requests.len() {
            prefs.workspaces[current_workspace].requests.push(request);
        } else {
            prefs.workspaces[current_workspace].requests[current_request_index] = request;
        }

        crate::preferences::utils::save_prefs(&prefs);
    }

    pub fn load_prefs(&self) {
        let prefs = crate::preferences::utils::get_prefs();

        for i in 0..self.workspaces_model.n_items() {
            self.workspaces_model.remove(i);
        }
        for i in 0..prefs.workspaces.len() {
            self.workspaces_model.append(&prefs.workspaces[i].name);
        }

        if prefs.workspaces.len() > 0 {
            self.load_workspace(&prefs.workspaces[0]);
        }
    }
}
// ANCHOR_END: template_callbacks

// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        let quit_action = SimpleAction::new("add_header", None);
        quit_action.connect_activate(clone!(@weak self as win => move |_, _| {
            win.create_header(KeyValuePair::default());
        }));
        obj.add_action(&quit_action);

        let quit_action = SimpleAction::new("add_query", None);
        quit_action.connect_activate(clone!(@weak self as win => move |_, _| {
            win.create_query(KeyValuePair::default());
        }));
        obj.add_action(&quit_action);

        self.set_response_text(String::from(""), None::<String>);

        self.add_header
            .connect_clicked(clone!(@weak self as win => move |_| {
                win.create_header(KeyValuePair::default());
            }));

        self.add_query
            .connect_clicked(clone!(@weak self as win => move |_| {
                win.create_query(KeyValuePair::default());
            }));

        self.format_body
            .connect_clicked(clone!(@weak self as win => move |_btn| {
                let formatted_text = format_json_string(win.body.text().to_string());
                win.body.set_text(formatted_text.as_str());
            }));

        self.create_workspace
            .connect_clicked(clone!(@weak self as win => move |_button| {
                let text = win.new_workspace_name.text();
                if !text.trim().is_empty() {
                    win.add_workspace(text.to_string());
                    win.new_workspace_name.set_text("");
                }
            }));

        self.create_request
            .connect_clicked(clone!(@weak self as win => move |_button| {
                let text = win.new_request_name.text();
                if !text.trim().is_empty() {
                    win.add_request(text.to_string());
                    win.new_request_name.set_text("");
                }
            }));

        self.workspaces
            .connect_activate(clone!(@weak self as win => move |_| {
                win.save_request();
            }));

        self.workspaces
            .connect_selected_item_notify(clone!(@weak self as win => move |_| {
                let workspace = get_prefs().workspaces[win.workspaces.selected() as usize].clone();
                win.load_workspace(&workspace);
            }));

        self.requests
            .connect_activate(clone!(@weak self as win => move |_| {
                win.save_request();
            }));

        self.requests
            .connect_selected_item_notify(clone!(@weak self as win => move |_| {
                let workspace = get_prefs().workspaces[win.workspaces.selected() as usize].clone();
                let mut request = preferences::Request::default();
                let index = win.requests.selected() as usize;
                if workspace.requests.len() > index {
                    request = workspace.requests[index].clone();
                }
                win.load_request(&request);
            }));

        self.load_prefs();
    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}

impl AdwApplicationWindowImpl for Window {}
