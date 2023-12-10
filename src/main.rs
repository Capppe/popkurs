mod device_widget;
mod device;
mod api_handler;
mod main_window;

use glib::once_cell::sync::Lazy;
use gtk::prelude::*;
use gtk::{glib, gio, Application, Grid, Label, Button};
use glib::clone;
use std::time::Duration;
use tokio::runtime::Runtime;

use api_handler::{get_devices, get_device};
use device_widget::DeviceWidget;

const APP_ID: &str = "ax.smartel.ui";
static RUNTIME: Lazy<Runtime> = 
    Lazy::new(|| Runtime::new().expect("Setting up tokio runtime needs to succeed"));

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

    let device_vec = add_devices(&top_layout).await;

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
        let dev_id = dev.id;
        let device = DeviceWidget::new(dev, None);
        layout.attach(&device.frame, index, column_index, 1, 1);
        index +=1;
        if index > 2 {
            column_index +=1;
            index = 0;
        }

        let (sender, reciever) = async_channel::bounded(1);
        
        RUNTIME.spawn(clone! (@strong sender => async move {
            loop {
                std::thread::sleep(Duration::from_secs(5));
                let response = get_device(dev_id).await;
                sender.send(response).await.expect("The channel needs to be open");
            }
        }));

        glib::spawn_future_local(async move {
            while let Ok(response) = reciever.recv().await {
                if let Ok(response) = response {
                    update_info(&device, response);
                }
            }
        });
    }
}

fn update_info(device: &DeviceWidget, new_data: device::Device) {
    let tot_kwh = format!("{:.4} KwH", &new_data.total_consumption.to_string());
    let power_state = &new_data.power;

    device.tot_kwh_value.set_text(&tot_kwh);
    device.state_value.set_text(power_state);
}

fn show_detailed_info(device: &DeviceWidget) {

}
