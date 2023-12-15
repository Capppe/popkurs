use gtk::prelude::*;
use gtk::{Label, Grid, Image, Frame, Box, Button};
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
            .css_classes(["device_widget_frame"])
            .build();

        let dev_image = Image::builder()
            .file(icon.unwrap_or("resources/icons/default_devices.png"))
            .build();

        let dev_name = Label::builder()
            .label(dev.name.as_str())
            .css_classes(["label"])
            .build();

        let dev_button = Button::builder()
            .margin_end(9)
            .margin_top(9)
            .margin_bottom(9)
            .margin_start(9)
            .hexpand(true)
            .css_classes(["device_button", "button"])
            .build();

        let state_label = Label::builder()
            .label("Power")
            .css_classes(["label"])
            .build();

        let tot_kwh_label = Label::builder()
            .label("Total Consumption")
            .css_classes(["label"])
            .build();

        let state_value = Label::builder()
            .label(dev.power)
            .css_classes(["label"])
            .build();

        let rounded_consumption = format!("{:.2} KwH", dev.total_consumption);
        let tot_kwh_value = Label::builder()
            .label(rounded_consumption)
            .css_classes(["label"])
            .build();

        state_layout.append(&state_label);
        state_layout.append(&state_value);

        kwh_layout.append(&tot_kwh_label);
        kwh_layout.append(&tot_kwh_value);

        layout.attach(&dev_name, 0, 1, 1, 1);
        layout.attach(&dev_image, 0, 2, 1, 1);
        layout.attach(&state_layout, 0, 3, 1, 1);
        layout.attach(&kwh_layout, 0, 4, 1, 1);

        dev_button.set_child(Some(&layout));

        Self { frame, state_value, tot_kwh_value, dev_button }
    }

    pub fn update_info(&self, data: &Device) {
        let tot_kwh = format!("{:.4} KwH", &data.total_consumption.to_string());
        let power_state = &data.power;

        self.tot_kwh_value.set_text(&tot_kwh);
        self.state_value.set_text(power_state);
    }
}
