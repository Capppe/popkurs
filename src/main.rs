//mod device_widget;
mod device;
mod api_handler;
mod main_window;
mod custom_widgets;

//use details_widget::DetailsWidget;
use gtk::prelude::*;
use gtk::{glib, Application};

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

    app.connect_activate(move |app| {
        MainWindow::show(app, window.clone());
    });

    app.run()    
}
