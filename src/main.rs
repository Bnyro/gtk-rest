mod window;

use gtk::gio;
use gtk::prelude::*;
use adw::Application;
use window::Window;

const APP_ID: &str = "com.bnyro.rest";

fn main() {
    gio::resources_register_include!("resources.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    let window = Window::new(&app);
    // Present window
    window.present();
}

