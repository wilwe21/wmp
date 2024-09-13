use gtk::prelude::*;
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

pub fn load_file(file: &str) -> gtk::Box {
    let streambox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let output = Command::new("ffprobe")
        .arg(&file)
        .output()
        .expect("failed to execute");
    let probe  = match str::from_utf8(&output.stderr){
        Ok(v) => v,
        Err(_) => &"error i chuj"
    };
    let Stream = gtk::Video::for_filename(Some(&file));
    streambox.append(&Stream);
    if probe != "error i chuj" {
        let find = ["artist          : ", "title           : ", "album           : ", "track           : ", "TRACKTOTAL      : ", "encoder         : ", "Duration: "];
        let meta = [&exline(&probe, &find[0])[find[0].len()..], &exline(&probe, &find[1])[find[1].len()..], &exline(&probe, &find[2])[find[2].len()..], &exline(&probe, &find[3])[find[3].len()..], &exline(&probe, &find[4])[find[4].len()..], &exline(&probe, &find[5])[find[5].len()..], &exline(&probe, &find[6])[find[6].len()..]];
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
        let nameBox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        nameBox.append(&TitleLab);
        nameBox.append(&ArtistLab);
        streambox.append(&nameBox);
        ArtistLab.add_css_class("artist");
        TitleLab.add_css_class("title");
        AlbumLab.add_css_class("album");
        TrackLab.add_css_class("track");
        EncodecLab.add_css_class("encodec");
        durLab.add_css_class("dur");
    }

    Stream.add_css_class("Stream");
    streambox
}
