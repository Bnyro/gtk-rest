use adw::subclass::prelude::AdwApplicationWindowImpl;
use adw::subclass::prelude::WidgetClassSubclassExt;
use glib::subclass::InitializingObject;
use gtk::subclass::prelude::ObjectImplExt;
use gtk::subclass::prelude::{ApplicationWindowImpl, ObjectImpl, ObjectSubclass};
use gtk::subclass::widget::{CompositeTemplateCallbacksClass, CompositeTemplateClass, WidgetImpl};
use gtk::subclass::window::WindowImpl;
use gtk::ListView;
use gtk::{glib, CompositeTemplate, DropDown, TemplateChild};
use gtk::{prelude::*, Button, Entry, HeaderBar};
use sourceview5::traits::BufferExt;

use crate::client::Request;

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
        );
        let resp = request.execute();
        match resp {
            Ok(response) => {
                let headers = response.headers().clone();
                let content_type = headers.get("Content-Type");
                let ct_split: Vec<&str> =
                    content_type.unwrap().to_str().unwrap().split(";").collect();
                let text = response.text().unwrap();
                self.set_response_text(text, Some(ct_split[0].to_string()));
            }
            Err(_) => {}
        }
    }
}

impl Window {
    pub fn set_response_text(&self, text: String, content_type: Option<String>) {
        println!("{:#?}", text);
        println!("{:#?}", content_type);
        let buffer = sourceview5::Buffer::new(None);
        buffer.set_highlight_syntax(true);
        buffer.set_highlight_matching_brackets(true);
        buffer.set_text(text.as_str());

        if let Some(content_type) = content_type {
            let language = sourceview5::LanguageManager::new()
                .guess_language(None::<String>, Some(content_type.as_str()));

            buffer.set_language(language.as_ref());

            if content_type.contains("json") {
                buffer.set_language(
                    sourceview5::LanguageManager::new()
                        .language("json")
                        .as_ref(),
                );
                let object: serde_json::Value = serde_json::from_str(text.as_str()).unwrap();
                let text = serde_json::to_string_pretty(&object).unwrap();
                buffer.set_text(text.as_str());
            }
        }

        if let Some(ref scheme) = sourceview5::StyleSchemeManager::new().scheme("classic-dark") {
            buffer.set_style_scheme(Some(scheme));
        }

        self.response.set_buffer(Some(&buffer));
    }
}
// ANCHOR_END: template_callbacks

// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        self.set_response_text(String::from(""), None::<String>);
    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}

impl AdwApplicationWindowImpl for Window {}
