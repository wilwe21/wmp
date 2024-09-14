use gtk::prelude::*;
use std::cell::RefCell;

use crate::load::load_file;
use crate::history::unwrap;

pub fn bar(parrent: gtk::CenterBox) -> gtk::CenterBox {
    let mBox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let scroll = gtk::ScrolledWindow::new();
    let history: RefCell<Vec<Vec<String>>> = RefCell::new(Vec::new());
    let sbox = unwrap(history.clone(), parrent.clone(), scroll.clone());
    sbox.add_css_class("scrollbox");
    scroll.set_propagate_natural_height(true);
    scroll.set_child(Some(&sbox));
    parrent.set_center_widget(Some(&scroll));
    let lab = gtk::Label::builder()
        .label("WMP")
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
    parrent.set_start_widget(Some(&mBox));
    mBox.add_css_class("topbar");
    let parr = parrent.clone();
    dialog.connect_response(move |dialog, response_type| {
        if response_type == gtk::ResponseType::Accept {
            let files = dialog.file();
            if let Some(file) = files {
                let path_temp = file.path().expect("Something's wrong");
                let path: &str = path_temp.to_str().unwrap().clone();
                let s = load_file(&path);
                for element in s {
                    if let Some(val) = element.downcast_ref::<String>() {
                        println!("string {}", val);
                        history.borrow_mut().push(vec!(path.to_string(), val.to_string()));
                        let hbox = unwrap(history.clone(), parr.clone(), scroll.clone());
                        scroll.set_child(Some(&hbox));
                        println!("{:?}", history);
                    } else if let Some(val) = element.downcast_ref::<gtk::Box>() {
                        parr.set_end_widget(Some(val));
                    } else {
                        println!("not a type");
                    }
                }
            }
        } else if response_type == gtk::ResponseType::DeleteEvent {
            println!("Cancel");
        }
        dialog.hide();
    });
    open.connect_clicked(move |_| dialog.show());
    parrent
}
