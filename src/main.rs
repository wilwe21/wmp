use gtk::prelude::*;
use gtk::gdk;

use std::process::Command;
use std::str;

fn exline(text: &str, delimiter: &str) -> String {
    if let Some(pos) = text.find(delimiter) {
        let new = &text[pos..].to_string();
        if let Some(end) = new.find("\n") {
            new[..end].to_string()
        } else {
            text[pos..].to_string()
        }
    } else {
        String::new()
    }
}

fn load_file(file: &str) -> gtk::Box {
    let streambox = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let output = Command::new("ffprobe")
        .arg(&file)
        .output()
        .expect("failed to execute");
    let probe  = match str::from_utf8(&output.stderr){
        Ok(v) => v,
        Err(e) => panic!("invalid UTF-8 sequence: {}", e)
    };
    let find = ["artist          : ", "title           : ", "album           : ", "track           : ", "TRACKTOTAL      : ", "encoder         : ", "Duration: "];
    let meta = [&exline(&probe, &find[0])[find[0].len()..], &exline(&probe, &find[1])[find[1].len()..], &exline(&probe, &find[2])[find[2].len()..], &exline(&probe, &find[3])[find[3].len()..], &exline(&probe, &find[4])[find[4].len()..], &exline(&probe, &find[5])[find[5].len()..], &exline(&probe, &find[6])[find[6].len()..]];
    let Stream = gtk::Video::for_filename(Some(&file));

    streambox.append(&Stream);
    let ArtistLab = gtk::Label::builder()
        .label(meta[0])
        .build();
    let TitleLab = gtk::Label::builder()
        .label(meta[1])
        .build();
    let AlbumLab = gtk::Label::builder()
        .label(meta[2])
        .build();
    let TrackLab = gtk::Label::builder()
        .label(format!("{}/{}", meta[3], meta[4]))
        .build();
    let EncodecLab = gtk::Label::builder()
        .label(meta[5])
        .build();
    let durLab = gtk::Label::builder()
        .label(meta[6])
        .build();
    streambox.append(&ArtistLab);
    streambox.append(&TitleLab);
    streambox.append(&AlbumLab);
    streambox.append(&TrackLab);
    streambox.append(&EncodecLab);
    streambox.append(&durLab);
    ArtistLab.add_css_class("artist");
    TitleLab.add_css_class("title");
    AlbumLab.add_css_class("album");
    TrackLab.add_css_class("track");
    EncodecLab.add_css_class("encodec");
    durLab.add_css_class("dur");
    Stream.add_css_class("Stream");
    streambox
}

fn on_active(app: &gtk::Application) {
    let mainBox = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let ls = Command::new("ls")
        .arg("/home/wilwe/Muzyka")
        .output()
        .expect("not a dir");
    let dir = match str::from_utf8(&ls.stdout) {
        Ok(v) => v,
        Err(e) => panic!("that's a wrong number: {}", e)
    };
    let file = "/home/wilwe/Muzyka/Korzenie/Korzenie.mp3";
    let s = load_file(&file);
    let lab = gtk::Label::builder()
        .label(dir)
        .build();
    mainBox.append(&s);
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
