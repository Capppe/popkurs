mod device_widget;
mod device;
mod api_handler;
mod main_window;

use device_widget::DeviceWidget;
use gtk::prelude::*;
use gtk::{glib, Application, Grid};

//use device_widget::DeviceWidget;
use api_handler::get_devices;


const APP_ID: &str = "ax.smartel.ui";

#[tokio::main]
async fn main() -> glib::ExitCode {
    // Initialize GTK
    gtk::init().expect("Failed to initialize GTK!");

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    let top_layout = Grid::builder()
        .column_homogeneous(true)
        .row_homogeneous(true)
        .build();

    add_devices(&top_layout).await;

    app.connect_activate(move |app| {
        main_window::build_main_window(&app, &top_layout)
    });

    app.run()    
}

async fn add_devices(layout: &Grid) {
    let devices = get_devices().await;
    let mut index = 0;
    let mut column_index = 0;

    for dev in devices {
        println!("Adding device: {}", dev.name);
        let device = DeviceWidget::new(dev, None);
        layout.attach(&device.frame, index, column_index, 1, 1);
        index +=1;
        if index > 2 {
            column_index +=1;
            index = 0;
        }
    }
}
