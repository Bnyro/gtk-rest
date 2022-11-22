use adw::subclass::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk::{gio, glib};

use crate::Window;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct GtkRestApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for GtkRestApplication {
        const NAME: &'static str = "GtkRestApplication";
        type Type = super::GtkRestApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for GtkRestApplication {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.instance();
            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<primary>q"]);
            obj.set_accels_for_action("app.about", &["<primary>a"]);
        }
    }

    impl ApplicationImpl for GtkRestApplication {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self) {
            let application = self.instance();
            // Get the current window or create one if necessary
            let window = if let Some(window) = application.active_window() {
                window
            } else {
                let window = Window::new(&*application);
                window.upcast()
            };

            // Ask the window manager/compositor to present the window
            window.present();
        }
    }

    impl GtkApplicationImpl for GtkRestApplication {}
    impl AdwApplicationImpl for GtkRestApplication {}
}

glib::wrapper! {
    pub struct GtkRestApplication(ObjectSubclass<imp::GtkRestApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl GtkRestApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::new::<GtkRestApplication>(&[
            ("application-id", &application_id),
            ("flags", flags),
        ])
    }

    fn setup_gactions(&self) {
        let quit_action = gio::SimpleAction::new("quit", None);
        quit_action.connect_activate(clone!(@weak self as app => move |_, _| {
            app.quit();
        }));
        self.add_action(&quit_action);

        let about_action = gio::SimpleAction::new("about", None);
        about_action.connect_activate(clone!(@weak self as app => move |_, _| {
            app.show_about();
        }));
        self.add_action(&about_action);
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        let about = adw::AboutWindow::builder()
            .transient_for(&window)
            .application_name("gtk-rest")
            .application_icon("com.bnyro.rest")
            .developer_name("Bnyro")
            .version("0.1.0")
            .developers(vec!["Bnyro".into()])
            .copyright("© 2022 Bnyro")
            .build();
        about.present();
    }
}
