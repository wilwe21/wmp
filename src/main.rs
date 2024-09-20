use gtk::prelude::*;
use gtk::gdk;

use std::process::Command;
use std::str;

mod load;
mod history;
mod menu;
mod top;

pub struct Music {
    file: String,
    title: String,
    author: String,
    album: String,
    track: u8,
    totaltrack: u8,
    encoder: String,
}
pub trait Ui {
    fn create(&self) -> gtk::Box;
}
impl Ui for Music {
    fn create(&self) -> gtk::Box {
        let main = gtk::Box::new(gtk::Orientation::Vertical, 1);
        let title = gtk::Label::builder()
            .label(self.title.clone())
            .build();
        main.append(&title);
        let artist = gtk::Label::builder()
            .label(self.author.clone())
            .build();
        main.append(&artist);
        main
    }
}

fn on_active(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .title("WMP")
        .application(app)
        .build();
    load_css();
    let mainBox = gtk::CenterBox::new();
    mainBox.set_orientation(gtk::Orientation::Vertical);
    let tbox = top::bar(mainBox);
    window.set_child(Some(&tbox));
    window.show();
}

fn load_css() {
    let display = gdk::Display::default().expect("Could not get default display.");
    let provider = gtk::CssProvider::new();
    let priority = gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;

    let css_content = include_str!("../css/main.css");
    provider.load_from_data(css_content);
    gtk::StyleContext::add_provider_for_display(&display, &provider, priority);
}


fn main() {
    let app = gtk::Application::builder()
        .application_id("com.github.wilwe21.wmp")
        .build();
    app.connect_activate(on_active);
    app.run();
}
