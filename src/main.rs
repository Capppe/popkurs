mod device;
mod api_handler;
mod main_window;
mod custom_widgets;

use gtk::prelude::*;
use gtk::{gdk, glib, Application, CssProvider};

use main_window::MainWindow;

const APP_ID: &str = "ax.smartel.ui";

// Builds the main window and adds all devices available from the api
#[tokio::main]
async fn main() -> glib::ExitCode {
    // Initialize GTK
    gtk::init().expect("Failed to initialize GTK!");

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    let main_window = MainWindow::new();
    let window = main_window.window;

    MainWindow::add_devices(&main_window.device_layout, main_window.details_sidebar).await;

    app.connect_startup(|_| load_css());
    app.connect_activate(move |app| {
        MainWindow::show(app, &window);
    });


    app.run()    
}

pub fn load_css() {
    let display = gdk::Display::default().expect("Could not connect to display");
    let provider = CssProvider::new();
    let priority = gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;
    provider.load_from_string(include_str!("../resources/style.css"));

    gtk::style_context_add_provider_for_display(&display, &provider, priority);
}
