use glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, DropDown};
use gtk::{prelude::*, Button, Entry, HeaderBar, Label};

use crate::client::Request;

// ANCHOR: object
// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/bnyro/rest/window.ui")]
pub struct Window {
    #[template_child]
    pub headerbar: TemplateChild<HeaderBar>,
    #[template_child]
    pub url: TemplateChild<Entry>,
    #[template_child]
    pub send: TemplateChild<Button>,
    #[template_child]
    pub response: TemplateChild<Label>,
    #[template_child]
    pub method: TemplateChild<DropDown>,
}
// ANCHOR_END: object

// ANCHOR: subclass
// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "GtkRestWindow";
    type Type = super::GtkRestWindow;
    type ParentType = gtk::ApplicationWindow;

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
        let request = Request::new(self.url.text().to_string(), self.method.selected());
        let resp = request.execute();
        // let client = reqwest::Client::new();
        if resp.is_ok() {
            let text = resp.unwrap().text().unwrap();
            println!("{:#?}", text);
            self.response.set_text(text.as_str());
        }
    }
}
// ANCHOR_END: template_callbacks

// Trait shared by all GObjects
impl ObjectImpl for Window {}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}
