use gtk::prelude::*;

use crate::load::load_file;

pub fn bar(parrent: gtk::Box) -> gtk::Box {
    let mBox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let lab = gtk::Label::builder()
        .label("test")
        .build();
    let open = gtk::Button::builder()
        .label("Open")
        .build();
    mBox.append(&lab);
    mBox.append(&open);
    let dialog = gtk::FileChooserDialog::builder()
        .title("Pick music file")
        .action(gtk::FileChooserAction::Open)
        .build();
    dialog.add_button("Open", gtk::ResponseType::Accept);
    parrent.append(&mBox);
    mBox.add_css_class("topbar");
    let parr = parrent.clone();
    dialog.connect_response(move |dialog, response_type| {
        if response_type == gtk::ResponseType::Accept {
            let files = dialog.file();
            if let Some(file) = files {
                let path_temp = file.path().expect("Something's wrong");
                let path: &str = path_temp.to_str().unwrap().clone();
                let s = load_file(&path);
                parr.append(&s);
            }
        } else if response_type == gtk::ResponseType::DeleteEvent {
            println!("Cancel");
        }
        dialog.hide();
    });
    open.connect_clicked(move |_| dialog.show());
    parrent
}
