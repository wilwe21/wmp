use gtk::prelude::*;
use std::process::Command;
use std::str;

use crate::class::Music;

fn get_name(path: &str) -> String {
    let vecpath: Vec<_> = path.split("/").collect();
    vecpath[vecpath.len()-1].to_string()
}

pub fn load_file(file: &str) -> Music {
    let streambox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let output = Command::new("ffprobe")
        .arg("-v")
        .arg("0")
        .arg("-show_entries")
        .arg("format")
        .arg("-of")
        .arg("json")
        .arg(&file)
        .output()
        .expect("failed to execute");
    let probe  = match str::from_utf8(&output.stderr){
        Ok(v) => v,
        Err(_) => &"error i chuj"
    };
    let Stream = gtk::Video::for_filename(Some(&file));
    streambox.append(&Stream);
    Stream.add_css_class("Stream");
    let mut vector = Music {
        streambox: streambox.clone(),
        file: file.clone().to_string(),
        title: "".to_string(),
        author: "".to_string(),
        album: "".to_string(),
        track: 0,
        totaltrack: 0,
        encoder: "".to_string(),
    };
    if probe != "error i chuj" {
        let probe = from_str(&probe).expect("reason");
        let nameBox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        streambox.append(&nameBox);
        match &exline(&probe, &find[1]) {
            Ok(res) => {
                let title = (&res[find[1].len()..]).to_string();
                vector.title = title;
            },
            Err(_) => {
                let title = get_name(&*file);
                vector.title = title;
            },
        }  
        match &exline(&probe, &find[0]) {
            Ok(res) => {
                let art = (&res[find[0].len()..]).to_string();
                vector.author = art;
            }
            Err(_) => {
                println!("no artist metadata")
            },
        }  
        match &exline(&probe, &find[2]) {
            Ok(res) => {
                let alb = (&res[find[2].len()..]).to_string();
                vector.album = alb;
            },
            Err(_) => {
                println!("no album metadata")
            },
        }  
        match &exline(&probe, &find[3]) {
            Ok(res) => {
                let track = &res[find[3].len()..].to_u32();
                vector.track = track
            },
            Err(_) => {
                println!("No track")
            },
        }  
        match &exline(&probe, &find[4]) {
            Ok(res) => {
                let track = &res[find[4].len()..].to_u32();
                vector.totaltrack = track
            },
            Err(_) => {
                println!("No track")
            },
        }  
        match &exline(&probe, &find[5]) {
            Ok(res) => {
                let enco = (&res[find[5].len()..]).to_string();
                vector.encoder = enco;
            },
            Err(_) => {
                println!("No encoder. What?")
            },
        }  
    }
    vector
}
