use gtk::prelude::*;
use gtk::{Label, Grid, Image, Frame, Align, Box, Button};
use crate::device::Device;

#[derive(Clone, Default)]
pub struct DeviceWidget {
    pub frame: Frame,
    pub state_value: Label,
    pub tot_kwh_value: Label,
    pub dev_button: Button,
}

impl DeviceWidget {
    pub fn new(dev: Device, icon: Option<&str>) -> Self {
        let layout = Grid::builder()
            .column_homogeneous(true)
            .row_homogeneous(true)
            .halign(Align::Fill)
            .valign(Align::Fill)
            .build();

        let state_layout = Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .homogeneous(true)
            .build();

        let kwh_layout = Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .homogeneous(true)
            .build();

        let frame = Frame::builder()
            .margin_start(5)
            .margin_end(5)
            .margin_top(5)
            .margin_bottom(5)
            .build();

        let dev_image = Image::builder()
            .file(icon.unwrap_or("resources/icons/default_devices.png"))
            .build();

        let dev_name = Label::builder()
            .label(dev.name.as_str())
            .build();

        let dev_button = Button::builder()
            .child(&dev_name)
            .margin_end(18)
            .margin_top(8)
            .margin_bottom(8)
            .margin_start(18)
            .build();

        let state_label = Label::new(Some("Power"));
        let tot_kwh_label = Label::new(Some("Total consumption"));

        let state_value = Label::new(Some(dev.power.as_str()));
        let rounded_consumption = format!("{:.2} KwH", dev.total_consumption);
        let tot_kwh_value = Label::new(Some(rounded_consumption.to_string().as_str()));

        state_layout.append(&state_label);
        state_layout.append(&state_value);

        kwh_layout.append(&tot_kwh_label);
        kwh_layout.append(&tot_kwh_value);

        layout.attach(&dev_button, 0, 0, 1, 1);
        layout.attach(&dev_image, 0, 2, 1, 1);
        layout.attach(&state_layout, 0, 3, 1, 1);
        layout.attach(&kwh_layout, 0, 4, 1, 1);

        frame.set_child(Some(&layout));

        Self { frame, state_value, tot_kwh_value, dev_button }
    }
}
