mod client;
mod window;
use adw::Application;
use gtk::gio;
use gtk::glib;
use gtk::prelude::*;
use window::Window;

const APP_ID: &str = "com.bnyro.rest";

fn main() {
    gio::resources_register_include!("resources.gresource").expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn show_about_dialog(app: &Application) {
    let window = app.active_window().unwrap();
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

fn build_ui(app: &Application) {
    activate(app);

    app.set_accels_for_action("app.quit", &["<primary>q"]);

    let action = gio::SimpleAction::new("about", None);
    action.connect_activate(glib::clone!(@weak app => move |_, _| {
        show_about_dialog(&app);
    }));
    app.add_action(&action);

    let quit_action = gio::SimpleAction::new("quit", None);
    quit_action.connect_activate(glib::clone!(@weak app => move |_, _| {
        app.quit();
    }));
    app.add_action(&quit_action);
}

fn activate(app: &Application) {
    // Get the current window or create one if necessary
    let window = if let Some(window) = app.active_window() {
        window
    } else {
        let window = Window::new(app);
        window.upcast()
    };

    // Ask the window manager/compositor to present the window
    window.present();
}
