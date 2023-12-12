use gtk::{Label, Frame, Grid, ListBox, ListBoxRow, Button};
use gtk::prelude::*;
use glib::clone;

use crate::{device::Device, api_handler};

#[derive(Clone)]
pub struct DetailsWidget {
    pub frame: Frame,
    pub top_layout: Grid,
    pub close_button: Button,
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

        let timestamps_label = Label::builder()
            .label("Timestamps")
            .build();

        let tot_usage_value = Label::builder()
            .label("")
            .build();

        let tot_usage_label = Label::builder()
            .label("Total consumption")
            .build();

        let dev_state_value = Label::builder()
            .label("")
            .build();

        let dev_state_label = Label::builder()
            .label("Power")
            .build();

        let dev_name = Label::builder()
            .label("")
            .build();

        let close_button = Button::builder()
            .label("X")
            .halign(gtk::Align::End)
            .build();

        let top_layout = Grid::builder()
            .build();
        top_layout.attach(&close_button, 0, 0, 1, 1);
        top_layout.attach(&dev_name, 0, 1, 1, 1);
        top_layout.attach(&dev_state_label, 0, 2, 1, 1);
        top_layout.attach(&dev_state_value, 0, 3, 1, 1);
        top_layout.attach(&tot_usage_label, 0, 4, 1, 1);
        top_layout.attach(&tot_usage_value, 0, 5, 1, 1);
        top_layout.attach(&timestamps_label, 0, 6, 1, 1);
        top_layout.attach(&timestamps_container, 0, 7, 1, 1);

        let frame = Frame::builder()
            .child(&top_layout)
            .visible(false)
            .build();

        close_button.connect_clicked(clone!(@strong frame => move |_| {
            frame.hide();
        }));


        Self { frame, top_layout, close_button, dev_name, dev_state_value, tot_usage_value, timestamps_container }
    }

    pub async fn show(&self, dev_id: u32) {
        println!("Button pressed, data: {}", dev_id);
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
        self.tot_usage_value.set_text(&device.total_consumption.to_string());

        //Need to empty the ListBox when a new device is displayed
        while let Some(child) = self.timestamps_container.last_child() {
            self.timestamps_container.remove(&child);
        }

        for data in device.consumption_data {
            let row = ListBoxRow::new();
            let text = format!(
            "Timestamp: {} \nPower Usage: {:.2} W \nEnergy Consumed: {:.6} kWh",
            data.timestamp, data.power_usage, data.energy_consumed
            );

            let label = Label::new(Some(&text));
            row.set_child(Some(&label));

            self.timestamps_container.append(&row);
        }
    }
}
