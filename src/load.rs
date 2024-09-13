use gtk::prelude::*;
use std::process::Command;
use std::str;

fn exline(text: &str, delimiter: &str) -> Result<String, &'static str> {
    if let Some(pos) = text.find(delimiter) {
        let new = &text[pos..].to_string();
        if let Some(end) = new.find("\n") {
            Ok(new[..end].to_string())
        } else {
            Ok(text[pos..].to_string())
        }
    } else {
        Err("empty")
    }
}

fn pathvalidate(text: &str) -> &str {
    /*let ve: Vec<_> = text.split("/").collect();
    let mut end: Vec<_> = Vec::new();
    for sar in &ve {
        if *sar != "" {
            end.push(format!("/\"{}\"", sar));
        }
    }
    println!("{}", end.join(""));*/
    text
}

pub fn load_file(file: &str) -> gtk::Box {
    let streambox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let output = Command::new("ffprobe")
        .arg(pathvalidate(&file))
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
        let nameBox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        streambox.append(&nameBox);
        match &exline(&probe, &find[1]) {
            Ok(res) => {
                let TitleLab = gtk::Label::builder()
                    .label(&res[find[1].len()..])
                    .build();
                nameBox.append(&TitleLab);
                TitleLab.add_css_class("title");
            },
            Err(_) => println!("Error: {}", &"error"),
        }  
        match &exline(&probe, &find[0]) {
            Ok(res) => {
                let ArtistLab = gtk::Label::builder()
                    .label(&res[find[0].len()..])
                    .build();
                nameBox.append(&ArtistLab);
                ArtistLab.add_css_class("artist");
            },
            Err(_) => println!("Error: {}", &"error"),
        }  
        match &exline(&probe, &find[2]) {
            Ok(res) => {
                let ArtistLab = gtk::Label::builder()
                    .label(&res[find[2].len()..])
                    .build();
                //nameBox.append(&TitleLab);
                ArtistLab.add_css_class("album");
            },
            Err(_) => println!("Error: {}", &"error"),
        }  
        match &exline(&probe, &find[3]) {
            Ok(res) => {
                match &exline(&probe, &find[4]) {
                    Ok(ress) => {
                        let TrackLab = gtk::Label::builder()
                            .label(format!("{}/{}", &res[find[3].len()..], &ress[find[4].len()..]))
                            .build();
                        //nameBox.append(&TrackLab);
                        TrackLab.add_css_class("track");
                    },
                    Err(_) => {
                        let TrackLab = gtk::Label::builder()
                            .label(&res[find[4].len()..])
                            .build();
                        //nameBox.append(&TruckLab);
                        TrackLab.add_css_class("track");
                    },
                }  
            },
            Err(_) => println!("Error: {}", &"error"),
        }  
        match &exline(&probe, &find[5]) {
            Ok(res) => {
                let EncoLab = gtk::Label::builder()
                    .label(&res[find[5].len()..])
                    .build();
                //nameBox.append(&EncoLab);
                EncoLab.add_css_class("encoder");
            },
            Err(_) => println!("Error: {}", &"error"),
          
        }  
        match &exline(&probe, &find[6]) {
            Ok(res) => {
                let DurLab = gtk::Label::builder()
                    .label(&res[find[6].len()..])
                    .build();
                //nameBox.append(&DurLab);
                DurLab.add_css_class("duration");
            },
            Err(_) => println!("Error: {}", &"error"),
        }; 
    }

    Stream.add_css_class("Stream");
    streambox
}
