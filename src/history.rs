use gtk::prelude::*;
use std::cell::RefCell;

use crate::load::load_file;

pub fn unwrap(history: RefCell<Vec<Vec<String>>>, parrent: gtk::CenterBox) -> gtk::Box {
    let mainBox = gtk::Box::new(gtk::Orientation::Vertical, 1);
    for e1 in history.clone().borrow_mut().iter_mut() {
        let parr = parrent.clone();
        let label = e1[1].clone();
        let path = e1[0].clone();
        let elm = gtk::Button::builder()
            .label(&label)
            .build();
        elm.connect_clicked(move |_| {
            println!("{}", &path);
            let s = load_file(&path);
            for element in s {
                if let Some(val) = element.downcast_ref::<gtk::Box>() {
                    parr.set_end_widget(Some(val));
                }
            }
        });
        mainBox.append(&elm);
    }
    mainBox
}
