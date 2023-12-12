use gtk::prelude::*;
use gtk::{Application, Window, HeaderBar, Grid, Button, Image, Label, Box};
use std::time::Duration;
use glib::once_cell::sync::Lazy;
use tokio::runtime::Runtime;
use glib::clone;

use crate::custom_widgets::{details_widget::DetailsWidget, device_widget::DeviceWidget};
use crate::api_handler::{get_device, get_devices};
use crate::device;

//Needed for multithreading with GTK
static RUNTIME: Lazy<Runtime> = 
    Lazy::new(|| Runtime::new().expect("Setting up tokio runtime needs to succeed"));

pub struct MainWindow {
    pub window: Window,
    pub top_layout: Box,
    pub device_layout: Grid,
    pub details_sidebar: DetailsWidget,
}

impl MainWindow {
    pub fn new() -> Self {
        let window = Window::builder()
            .title("Smart El")
            .build();

        let header_bar = HeaderBar::builder()
            .show_title_buttons(true)
            .build();

        let top_layout = Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .homogeneous(false)
            .halign(gtk::Align::Fill)
            .build();

        let device_layout = Grid::builder()
            .column_homogeneous(true)
            .row_homogeneous(true)
            .build();

        let details_sidebar = DetailsWidget::new();

        let pref_button = Self::build_custom_button("Preferences", "resources/icons/setting.png");

        let add_button = Self::build_custom_button("Add", "resources/icons/add.png");

        header_bar.pack_end(&pref_button);
        header_bar.pack_start(&add_button);

        top_layout.append(&device_layout);
        top_layout.append(&details_sidebar.frame);
     
        window.set_titlebar(Some(&header_bar));
        window.set_child(Some(&top_layout));

        Self { window, top_layout, device_layout, details_sidebar }
    }

    pub fn show(app: &Application, window: Window) {
        app.add_window(&window);
        window.present();
    }

    //Gets available devices from api then inserts them into the layout
    pub async fn add_devices(layout: &Grid, details_widget: DetailsWidget) {
        let devices = get_devices().await;
        let mut index = 0;
        let mut column_index = 0;

        for dev in devices {
            let dev_id = dev.id;
            let device = DeviceWidget::new(dev, None);

            device.frame.set_hexpand(true);
            layout.attach(&device.frame, index, column_index, 1, 1);
            index +=1;
            if index > 2 {
                column_index +=1;
                index = 0;
            }

            device.dev_button.connect_clicked(clone!(@strong details_widget => move |_| {
                let details_widget_clone = details_widget.clone();
                let (sender, reciever) = async_channel::bounded(1);

                RUNTIME.spawn(clone! (@strong sender => async move {
                    let response = get_device(dev_id).await;
                    sender.send(response).await.expect("The channel needs to be open");
                }));

                glib::spawn_future_local(async move {
                    let Ok(response) = reciever.recv().await else{todo!()};
                        if let Ok(response) = response {
                            details_widget_clone.show(response.id).await;
                        }
                });
            }));

            spawn_updater_thread(dev_id, device);
        }
    }


    fn build_custom_button(text: &str, path: &str) -> Button {
        let image = Image::from_file(path);

        let label = Label::new(Some(text));

        let container = Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(5)
            .build();

        let button = Button::new();

        container.append(&label);
        container.append(&image);

        button.set_child(Some(&container));

        return button;
    }

}

//Fetches new data from api asynchronously every 5 seconds and sends it to update_info()
fn spawn_updater_thread(device_id: u32, device_widget: DeviceWidget) {
    let (sender, reciever) = async_channel::bounded(1);
    
    RUNTIME.spawn(clone! (@strong sender => async move {
        loop {
            std::thread::sleep(Duration::from_secs(5));
            let response = get_device(device_id).await;
            sender.send(response).await.expect("The channel needs to be open");
        }
    }));

    glib::spawn_future_local(async move {
        while let Ok(response) = reciever.recv().await {
            if let Ok(response) = response {
                update_info(&device_widget, response);
            }
        }
    });

}

//Updates the devices in the UI with new data
fn update_info(device: &DeviceWidget, new_data: device::Device) {
    let tot_kwh = format!("{:.4} KwH", &new_data.total_consumption.to_string());
    let power_state = &new_data.power;

    device.tot_kwh_value.set_text(&tot_kwh);
    device.state_value.set_text(power_state);
}
