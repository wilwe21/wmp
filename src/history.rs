use gtk::prelude::*;
use std::cell::RefCell;
use std::fs;

use serde_json::from_str;
use crate::load::load_file;
use crate::top::savehis;

pub fn unwrap(parrent: gtk::CenterBox, scrBox: gtk::ScrolledWindow) -> gtk::Box {
    let mainBox = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let history: RefCell<Vec<Vec<String>>> = RefCell::new(Vec::new());
    match fs::read_to_string("/home/wilwe/.config/wmp/history") {
        Ok(res) => {
            let sus = from_str(&res).expect("reson");
            *history.borrow_mut() = sus;
        },
        Err(_) => {
            println!("No history file found");
        }
    };
    for e1 in history.clone().borrow_mut().iter_mut() {
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
            savehis(his.clone());
            let BBox = s[0].downcast_ref::<gtk::Box>();
            parr.set_end_widget(Some(BBox.expect("reason")));
        });
        mainBox.append(&elm);
    }
    mainBox
}
