mod imp;

use gtk::{gio, glib};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
    @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::new::<Window>(&[("application", application)])
    }
}

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
    pub method: TemplateChild<DropDown>,
    #[template_child]
    pub body: TemplateChild<Entry>,
    #[template_child]
    pub headers: TemplateChild<ListView>,
    #[template_child]
    pub queries: TemplateChild<ListView>,
    #[template_child]
    pub response: TemplateChild<sourceview5::View>,

    pub headers: RefCell<Option<gio::ListStore>>,
}
// ANCHOR_END: object
