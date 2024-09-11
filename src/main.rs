use gtk::prelude::*;
use gtk::gdk;

use ffmpeg_next::codec::{audio::Audio, Context};
use ffmpeg_next::format::{input, Stream, probe::Probe};
use ffmpeg_next::media::packet::Packet;

fn on_active(app: &gtk::Application) {
    let mainBox = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let label = gtk::Label::builder()
        .label("sss")
        .build();
    mainBox.append(&label);
    let window = gtk::ApplicationWindow::builder()
        .title("WMP")
        .application(app)
        .build();
    load_css();
    window.set_child(Some(&mainBox));
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
