use gtk::prelude::*;
use std::cell::RefCell;
use std::fs;

use serde_json::from_str;
use crate::load::load_file;
use crate::history::unwrap;
use crate::menu::menu;

pub fn savehis(history: RefCell<Vec<Vec<String>>>) {
    fs::write("/home/wilwe/.config/wmp/history", format!("{:?}", history.borrow()));
    println!("saved");
}

pub fn bar(parrent: gtk::CenterBox) -> gtk::CenterBox {
    let history: RefCell<Vec<Vec<String>>> = RefCell::new(Vec::new());
    let hhh = history.clone();
    match fs::read_to_string("/home/wilwe/.config/wmp/history") {
        Ok(res) => {
            let sus = from_str(&res).expect("reson");
            *history.borrow_mut() = sus;
        },
        Err(_) => {
            println!("No history file found");
        }
    };
    let mBox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let scroll = gtk::ScrolledWindow::new();
    let sbox = unwrap(history.clone(), parrent.clone(), scroll.clone());
    sbox.add_css_class("scrollbox");
    scroll.set_propagate_natural_height(true);
    scroll.set_child(Some(&sbox));
    parrent.set_center_widget(Some(&scroll));
    let main = gtk::Button::builder()
        .label("WMP")
        .build();
    let hitbut = gtk::Button::builder()
        .label("History")
        .build();
    let open = gtk::Button::builder()
        .label("Open")
        .build();
    let scrclon = scroll.clone();
    let scrclon2 = scroll.clone();
    main.connect_clicked(move |_| {
        let mainb = menu();
        scrclon.set_child(Some(&mainb));
    });
    let hiscoln = history.clone();
    let hissclon = hiscoln.clone();
    let parclon = parrent.clone();
    hitbut.connect_clicked(move |_| {
        match fs::read_to_string("/home/wilwe/.config/wmp/history") {
            Ok(res) => {
                let vec = from_str(&res).expect("reson");
                *hiscoln.borrow_mut() = vec;
            },
            Err(_) => {
                println!("Empty history file");
            }
        };
        let ssbox = unwrap(hiscoln.clone(), parclon.clone(), scrclon2.clone());
        scrclon2.set_child(Some(&ssbox));
    });
    mBox.append(&main);
    mBox.append(&hitbut);
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
                let tit = s[1].downcast_ref::<String>();
                hissclon.borrow_mut().push(vec![path.to_string(), tit.expect("reason").to_string()]);
                savehis(hissclon.clone());
                let BBox = s[0].downcast_ref::<gtk::Box>();
                parr.set_end_widget(Some(BBox.expect("reason")));
            }
        }
        dialog.hide();
    });
    open.connect_clicked(move |_| dialog.show());
    parrent
}
