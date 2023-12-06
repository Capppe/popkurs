use gtk::prelude::*;
use gtk::{Label, Grid, Image};

pub struct DeviceWidget {
    pub layout: Grid,
}

impl DeviceWidget {

pub fn new() -> Self {
    let layout = Grid::builder()
        .column_homogeneous(true)
        .row_homogeneous(true)
        .build();

    let dev_image = Image::builder()
        .file("src/resources/icons/fridge.png")
        .build();

    let label1 = Label::new(Some("State"));
    let label2 = Label::new(Some("KwH"));

    layout.attach(&dev_image, 0, 0, 1, 1);
    layout.attach(&label1, 0, 1, 1, 1);
    layout.attach(&label2, 0, 2, 1, 1);

    Self { layout }
}

}
