use std::cell::RefCell;
use std::rc::Rc;
use std::thread;

use adw::subclass::prelude::AdwApplicationWindowImpl;
use adw::subclass::prelude::WidgetClassSubclassExt;
use glib::subclass::InitializingObject;
use gtk::glib::clone;
use gtk::glib::MainContext;
use gtk::glib::PRIORITY_DEFAULT;
use gtk::subclass::prelude::ObjectImplExt;
use gtk::subclass::prelude::{ApplicationWindowImpl, ObjectImpl, ObjectSubclass};
use gtk::subclass::widget::{CompositeTemplateCallbacksClass, CompositeTemplateClass, WidgetImpl};
use gtk::subclass::window::WindowImpl;
use gtk::MenuButton;
use gtk::StringList;
use gtk::{glib, CompositeTemplate, DropDown, TemplateChild};
use gtk::{prelude::*, Box, Button, Entry};
use sourceview5::traits::BufferExt;

use crate::client::Request;
use crate::preferences;
use crate::preferences::Preferences;
use crate::preferences::utils::get_prefs;
use crate::preferences::utils::save_prefs;
use crate::preferences::KeyValuePair;
use crate::utils::format_json_string;
use crate::widgets::kvpair::KvPair;
use crate::widgets::request_row::RequestRow;

// ANCHOR: object
// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/bnyro/rest/window.ui")]
pub struct Window {
    #[template_child]
    pub workspaces: TemplateChild<DropDown>,
    #[template_child]
    pub workspaces_model: TemplateChild<StringList>,
    #[template_child]
    pub new_workspace_name: TemplateChild<Entry>,
    #[template_child]
    pub requests: TemplateChild<Box>,
    #[template_child]
    pub new_request_name: TemplateChild<Entry>,
    #[template_child]
    pub url: TemplateChild<Entry>,
    #[template_child]
    pub method: TemplateChild<DropDown>,
    #[template_child]
    pub body: TemplateChild<sourceview5::View>,
    #[template_child]
    pub headers: TemplateChild<Box>,
    #[template_child]
    pub queries: TemplateChild<Box>,
    #[template_child]
    pub response: TemplateChild<sourceview5::View>,
    pub header_pairs: RefCell<Vec<KeyValuePair>>,
    pub query_pairs: RefCell<Vec<KeyValuePair>>,
    pub request_index: RefCell<usize>,
    pub preferences: RefCell<Preferences>
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
    fn handle_send(&self, _button: &Button) {
        let request = Request::new(
            self.url.text().to_string(),
            self.get_body_text(),
            self.method.selected(),
            self.header_pairs.clone().take(),
            self.query_pairs.clone().take(),
        );

        self.save_request();

        let (sender, receiver) = MainContext::channel::<(String, Option<String>)>(PRIORITY_DEFAULT);

        // The long running operation runs now in a separate thread
        thread::spawn(move || {
            let response = request.execute();

            if let Ok(response) = response {
                let content_type = response.content_type().to_string();
                let text = response.into_string();
                
                sender.send((text.unwrap(), Some(content_type))).expect("Error sending data");
            }
        });
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
    #[template_callback]
    fn handle_create_workspace(&self, _button: &Button) {
        let text = self.new_workspace_name.text();
        if !text.trim().is_empty() {
            self.add_workspace(text.to_string());
            self.new_workspace_name.set_text("");
        }
    }

    #[template_callback]
    fn handle_create_request(&self, _button: &Button) {
        let text = self.new_request_name.text();
        if text.trim().is_empty() {
            return;
        };

        // create the new request
        self.add_request(text.to_string(), false);
        self.new_request_name.set_text("");
    }

    #[template_callback]
    fn handle_format_body(&self, _button: &Button) {
        let formatted_text = format_json_string(self.get_body_text());
        self.body.buffer().set_text(formatted_text.as_str());
    }

    #[template_callback]
    fn handle_create_query(&self, _button: &Button) {
        self.create_query(&KeyValuePair::default());
    }

    #[template_callback]
    fn handle_add_header(&self, _button: &Button) {
        self.create_header(KeyValuePair::default());
    }

    #[template_callback]
    fn handle_add_query(&self, _button: &Button) {
        self.create_query(&KeyValuePair::default());
    }

    #[template_callback]
    fn handle_save(&self, _button: &MenuButton) {
        self.save_request();
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
        self.header_pairs.borrow_mut().push(pair.clone());
        let on_change = clone!(@weak self as win => move |index, kvp| {
            win.header_pairs.borrow_mut()[index] = kvp;
        });
        let mut kv_pair = KvPair::new(index, pair);
        let child = kv_pair.build(&self.headers, Rc::new(on_change));
        self.headers.append(&child);
    }

    pub fn create_query(&self, pair: &KeyValuePair) {
        let index = self.query_pairs.borrow().len();
        self.query_pairs.borrow_mut().push(pair.clone());
        let on_change = clone!(@weak self as win => move |index, kvp| {
            win.query_pairs.borrow_mut()[index] = kvp;
        });
        let mut kv_pair = KvPair::new(index, pair.clone());
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
        self.workspaces_model.append(workspace_name.as_str());
        let mut workspace = preferences::Workspace::default();
        workspace.name = workspace_name;
        let mut request = preferences::Request::default();
        request.name = String::from("Default");
        workspace.requests.push(request);

        // save to the settings
        self.preferences.borrow_mut().workspaces.push(workspace.clone());
        crate::preferences::utils::save_prefs(&self.preferences.borrow());

        // select the newly created workspace
        self.workspaces
            .set_selected(self.workspaces_model.n_items() - 1);

        // load the new workspace
        self.load_workspace(&workspace);
    }

    pub fn add_request(&self, request_name: String, clean: bool) {
        // check whether the request name already exists
        if !clean {
            let requests = self.preferences.borrow().workspaces[self.workspaces.selected() as usize]
                .requests
                .clone();
            for i in 0..requests.len() {
                if requests[i].name == request_name {
                    return;
                }
            }
        }

        // create the new request
        let request_row = RequestRow::new(request_name.clone());
        let child = request_row.build(
            clone!(@weak self as win => move |request_name, _container| {
                win.save_request();
                let request_index = win.get_request_index(request_name);
                win.request_index.replace(request_index);
                let request = win.get_request_by_index(request_index);
                win.load_request(&request);
            }),
            clone!(@weak self as win => move |request_name, container| {
                let request_index = win.get_request_index(request_name);
                win.delete_request_by_index(request_index);
                win.requests.remove(container);
            }),
        );
        self.requests.append(&child);

        let mut request = preferences::Request::default();
        request.name = request_name.clone();

        // load and save the new created request
        self.load_request(&request);

        self.save_request();
    }

    pub fn get_request_index(&self, request_name: String) -> usize {
        let workspace = self.preferences.borrow().workspaces[self.workspaces.selected() as usize].clone();
        let index = workspace
            .requests
            .iter()
            .position(|request| request.name == request_name);
        match index {
            Some(index) => return index,
            None => return 0,
        }
    }

    pub fn get_request_by_index(&self, index: usize) -> preferences::Request {
        let workspace = self.preferences.borrow().workspaces.clone();
        workspace[self.workspaces.selected() as usize].requests[index].clone()
    }

    pub fn delete_request_by_index(&self, index: usize) {
        self.preferences.borrow_mut().workspaces[self.workspaces.selected() as usize]
            .requests
            .remove(index);
        save_prefs(&self.preferences.borrow());
    }

    pub fn load_request(&self, request: &preferences::Request) {
        self.save_request();

        self.body.buffer().set_text(request.body.as_str());

        self.url.set_text(request.target_url.as_str());
        self.method.set_selected(request.method);

        // remove all headers in the UI
        while let Some(child) = self.headers.first_child() {
            self.headers.remove(&child);
        }
        self.header_pairs.borrow_mut().clear();

        for i in 0..request.headers.len() {
            self.create_header(request.headers[i].clone());
        }

        // remove all queries in the UI
        while let Some(child) = self.queries.first_child() {
            self.queries.remove(&child);
        }
        self.query_pairs.borrow_mut().clear();

        for i in 0..request.queries.len() {
            self.create_query(&request.queries[i].clone());
        }
    }

    pub fn load_workspace(&self, workspace: &preferences::Workspace) {
        while let Some(child) = self.requests.first_child() {
            self.requests.remove(&child);
        }

        for i in 0..workspace.requests.len() {
            self.add_request(workspace.requests[i].name.clone(), true);
        }

        self.load_request(&workspace.requests[0]);
    }

    pub fn get_request(&self) -> preferences::Request {
        let mut request = preferences::Request::default();
        request.headers = self.header_pairs.clone().take();
        request.queries = self.query_pairs.clone().take();
        request.body = self.get_body_text();
        request.target_url = self.url.text().to_string();
        // let workspace = get_prefs().workspaces[self.workspaces.selected() as usize].clone();
        let workspace = get_prefs().workspaces[0].clone();
        request.name = workspace.requests[self.request_index.clone().take()]
            .name
            .clone();
        request.method = self.method.selected();
        request
    }

    pub fn save_request(&self) {
        let workspace_index = self.workspaces.selected() as usize;
        if workspace_index == 4294967295 { return }

        let request = &self.get_request();

        if request.name.trim().is_empty() {
            return;
        };

        let mut added = false;
        for i in 0..self.preferences.borrow().workspaces[workspace_index].requests.len() {
            if self.preferences.borrow().workspaces[workspace_index].requests[i].name == request.clone().name {
                self.preferences.borrow_mut().workspaces[workspace_index].requests[i] = request.clone();
                added = true;
            }
        }
        if !added {
            self.preferences.borrow_mut().workspaces[workspace_index]
                .requests
                .push(request.clone());
        }

        crate::preferences::utils::save_prefs(&self.preferences.borrow());
    }

    pub fn load_prefs(&self) {
        for i in 0..self.workspaces_model.n_items() {
            self.workspaces_model.remove(i);
        }
        for i in 0..self.preferences.borrow().workspaces.len() {
            self.workspaces_model.append(&self.preferences.borrow().workspaces[i].name);
        }

        if self.preferences.borrow().workspaces.len() > 0 {
            self.load_workspace(&self.preferences.borrow().workspaces[0]);
        }
    }

    pub fn init_body(&self) {
        let buffer = sourceview5::Buffer::new(None);
        let language = sourceview5::LanguageManager::new().language("json");
        buffer.set_highlight_syntax(true);
        buffer.set_highlight_matching_brackets(true);
        buffer.set_language(language.as_ref());
        if let Some(ref scheme) = sourceview5::StyleSchemeManager::new().scheme("classic-dark") {
            buffer.set_style_scheme(Some(scheme));
        }
        self.body.set_buffer(Some(&buffer));
    }

    pub fn get_body_text(&self) -> String {
        let buffer = self.body.buffer();
        buffer
            .text(&buffer.start_iter(), &buffer.end_iter(), false)
            .as_str()
            .to_string()
    }
}
// ANCHOR_END: template_callbacks

// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        self.set_response_text(String::from(""), None::<String>);

        self.workspaces
            .connect_activate(clone!(@weak self as win => move |_| {
                win.save_request();
            }));

        self.workspaces
            .connect_selected_item_notify(clone!(@weak self as win => move |_| {
                if win.preferences.borrow().workspaces.is_empty() { return };
                let workspace = win.preferences.borrow().workspaces[win.workspaces.selected() as usize].clone();
                win.load_workspace(&workspace);
            }));

        self.init_body();

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
