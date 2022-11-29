mod application;
mod client;
mod preferences;
mod utils;
mod widgets;
mod window;
use application::GtkRestApplication;
use gtk::gdk::Display;
use gtk::gio;
use gtk::prelude::*;
use gtk::CssProvider;
use gtk::StyleContext;
use window::Window;

const APP_ID: &str = "com.bnyro.rest";

fn main() {
    gio::resources_register_include!("resources.gresource").expect("Failed to register resources.");

    // Create a new GtkApplication. The application manages our main loop,
    // application windows, integration with the window manager/compositor, and
    // desktop features such as file opening and single-instance applications.
    let app = GtkRestApplication::new(APP_ID, &gio::ApplicationFlags::empty());

    // Attach the CSS when the app is loaded
    app.connect_startup(|_| load_css());

    // Run the application. This function will block until the application
    // exits. Upon return, we have our exit code to return to the shell. (This
    // is the code you see when you do `echo $?` after running a command in a
    // terminal.
    std::process::exit(app.run());
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("style.css"));

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
