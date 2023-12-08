use gtk::prelude::*;
use gtk::{Application, Window, HeaderBar, Grid, Button, Image, Label, Box};

pub fn build_main_window(app: &Application, top_layout: &Grid) {
    let window = Window::builder()
        .title("Smart El")
        .build();

    let header_bar = HeaderBar::builder()
        .show_title_buttons(true)
        .build();

    let pref_button = build_custom_button("Preferences", "resources/icons/setting.png");

    let add_button = build_custom_button("Add", "resources/icons/add.png");
/*
    let top_layout = Grid::builder()
        .column_homogeneous(true)
        .row_homogeneous(true)
        .build();
*/
    header_bar.pack_end(&pref_button);
    header_bar.pack_start(&add_button);
    
    window.set_titlebar(Some(&header_bar));
    window.set_child(Some(top_layout));

    app.add_window(&window);
    window.present();
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
