use gtk::prelude::*;
use gtk::{Label, Grid, Image, Frame, Align, Box};
use crate::device::Device;

pub struct DeviceWidget {
    pub frame: Frame,
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
        .build();

    let dev_image = Image::builder()
        .file(icon.unwrap_or("resources/icons/default_devices.png"))
        .build();

    let dev_name = Label::builder()
        .label(dev.name.as_str())
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

    layout.attach(&dev_name, 0, 0, 1, 1);
    layout.attach(&dev_image, 0, 1, 1, 1);
    layout.attach(&state_layout, 0, 2, 1, 1);
    layout.attach(&kwh_layout, 0, 4, 1, 1);

    frame.set_child(Some(&layout));

    Self { frame }
}

}

