use gtk::prelude::*;
use std::cell::RefCell;

use crate::load::load_file;

pub fn unwrap(history: RefCell<Vec<Vec<String>>>, parrent: gtk::CenterBox, scrBox: gtk::ScrolledWindow) -> gtk::Box {
    let mainBox = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let hist = history.clone();
    for e1 in hist.clone().borrow_mut().iter_mut() {
        let his = history.clone();
        let parr = parrent.clone();
        let scroll = scrBox.clone();
        let label = e1[1].clone();
        let path = e1[0].clone();
        let elm = gtk::Button::builder()
            .label(&label)
            .build();
        elm.connect_clicked(move |_| {
            let s = load_file(&path);
            let tit = s[1].downcast_ref::<String>();
            his.borrow_mut().push(vec!(path.to_string(), tit.expect("reason").to_string()));
            let hbox = unwrap(his.clone(), parr.clone(), scroll.clone());
            scroll.set_child(Some(&hbox));
            let BBox = s[0].downcast_ref::<gtk::Box>();
            parr.set_end_widget(Some(BBox.expect("reason")));
        });
        mainBox.append(&elm);
    }
    mainBox
}
