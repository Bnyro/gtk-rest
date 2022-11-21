use adw::subclass::prelude::AdwApplicationWindowImpl;
use adw::subclass::prelude::WidgetClassSubclassExt;
use glib::subclass::InitializingObject;
use gtk::subclass::prelude::{ApplicationWindowImpl, ObjectImpl, ObjectSubclass};
use gtk::subclass::widget::{CompositeTemplateCallbacksClass, CompositeTemplateClass, WidgetImpl};
use gtk::subclass::window::WindowImpl;
use gtk::{glib, CompositeTemplate, DropDown, TemplateChild};
use gtk::{prelude::*, Button, Entry, HeaderBar};
use sourceview5::traits::BufferExt;

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
    pub response: TemplateChild<sourceview5::View>,
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
        let request = Request::new(self.url.text().to_string(), self.method.selected());
        let resp = request.execute();
        // let client = reqwest::Client::new();
        if resp.is_ok() {
            let text = resp.unwrap().text().unwrap();
            println!("{:#?}", text);
            let buffer = sourceview5::Buffer::new(None);
            buffer.set_highlight_syntax(true);
            buffer.set_text(text.as_str());
            if let Some(ref language) = sourceview5::LanguageManager::new().language("json") {
                buffer.set_language(Some(language));
            }
            if let Some(ref scheme) =
                sourceview5::StyleSchemeManager::new().scheme("solarized-light")
            {
                buffer.set_style_scheme(Some(scheme));
            }
            buffer.set_highlight_matching_brackets(true);

            self.response.set_buffer(Some(&buffer));
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

impl AdwApplicationWindowImpl for Window {}
