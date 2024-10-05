use gtk::prelude::*;

pub fn menu() -> gtk::Box {
    let menuBox = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let lab = gtk::Label::builder()
        .label("chuj")
        .build();
    menuBox.append(&lab);
    menuBox
}
