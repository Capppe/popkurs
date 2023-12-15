use gtk::{Label, Frame, Grid, Box, ListBox, ListBoxRow, Button};
use gtk::prelude::*;
use glib::clone;

use crate::{device::Device, api_handler};

#[derive(Clone)]
pub struct DetailsWidget {
    pub frame: Frame,
    pub top_layout: Grid,
    pub name_and_close: Box,
    pub dev_name: Label,
    pub dev_state_value: Label,
    pub tot_usage_value: Label,
    pub timestamps_container: ListBox,
}

impl DetailsWidget {
    pub fn new() -> Self {
        let timestamps_container = ListBox::builder()
            .margin_bottom(3)
            .margin_top(3)
            .margin_start(5)
            .margin_end(5)
            .build();
        timestamps_container.add_css_class("timestamp_list");

        let timestamps_label = Label::builder()
            .label("Timestamps")
            .css_classes(["label"])
            .build();

        let tot_usage_value = Label::builder()
            .label("")
            .css_classes(["label"])
            .build();

        let tot_usage_label = Label::builder()
            .label("Total consumption:")
            .css_classes(["label"])
            .build();

        let dev_state_value = Label::builder()
            .label("")
            .css_classes(["label"])
            .halign(gtk::Align::Start)
            .build();

        let dev_state_label = Label::builder()
            .label("Power: ")
            .halign(gtk::Align::End)
            .css_classes(["label"])
            .build();

        let dev_state_box = Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .homogeneous(true)
            .spacing(10)
            .build();
        dev_state_box.append(&dev_state_label);
        dev_state_box.append(&dev_state_value);

        let dev_name = Label::builder()
            .halign(gtk::Align::Start)
            .hexpand(true)
            .margin_start(15)
            .label("")
            .css_classes(["label"])
            .build();
        dev_name.add_css_class("name_label");

        let close_button = Button::builder()
            .label("X")
            .halign(gtk::Align::End)
            .margin_top(3)
            .margin_bottom(3)
            .margin_end(5)
            .css_classes(["close_button", "button"])
            .build();
        close_button.add_css_class("close_button");

        let name_and_close = Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .homogeneous(false)
            .build();
        name_and_close.append(&dev_name);
        name_and_close.append(&close_button);
        name_and_close.add_css_class("name_and_close");

        let top_layout = Grid::builder()
            .row_spacing(10)
            .build();
        top_layout.attach(&name_and_close, 0, 0, 1, 1);
        top_layout.attach(&dev_state_box, 0, 1, 1, 1);
        top_layout.attach(&tot_usage_label, 0, 4, 1, 1);
        top_layout.attach(&tot_usage_value, 0, 5, 1, 1);
        top_layout.attach(&timestamps_label, 0, 6, 1, 1);
        top_layout.attach(&timestamps_container, 0, 7, 1, 1);

        let frame = Frame::builder()
            .child(&top_layout)
            .visible(false)
            .build();
        frame.add_css_class("frame");

        close_button.connect_clicked(clone!(@strong frame => move |_| {
            frame.set_visible(false);
        }));

        Self { frame, top_layout, name_and_close, dev_name, dev_state_value, tot_usage_value, timestamps_container }
    }

    pub async fn show(&self, dev_id: u32) {
        match api_handler::get_device(dev_id).await {
            Ok(response) => self.append_consumption_data(response),
            Err(_err) => return,
        };

        self.frame.set_visible(true);
    }

    //Gets detailed data from API and inserts the data into respective labell
    fn append_consumption_data(&self, device: Device) {
        println!("Button pressed, data: {}", device.name);
        self.dev_name.set_text(&device.name);
        self.dev_state_value.set_text(&device.power);
        let tot_usage_str = format!("{} kWh", device.total_consumption.to_string());
        self.tot_usage_value.set_text(&tot_usage_str);

        //Need to empty the ListBox when a new device is displayed
        while let Some(child) = self.timestamps_container.last_child() {
            self.timestamps_container.remove(&child);
        }

        for data in device.consumption_data {
            let row = ListBoxRow::new();
            let mut parts = data.timestamp.split('T');
            let date = parts.next().unwrap_or("");
            let time = parts.next().unwrap_or("").trim_end_matches('Z');
            let text = format!("Date: {}\nTime: {}\nPower Usage: {} W \nEnergy consumed: {}\n", date, time, data.power_usage, data.energy_consumed);

            let label = Label::builder()
                .label(text)
                .css_classes(["label"])
                .build();
            row.set_child(Some(&label));

            self.timestamps_container.append(&row);
        }
    }
}
