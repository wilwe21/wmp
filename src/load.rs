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

fn get_name(path: &str) -> String {
    let vecpath: Vec<_> = path.split("/").collect();
    vecpath[vecpath.len()-1].to_string()
}

pub fn load_file(file: &str) -> Vec<Box<dyn std::any::Any>> {
    let mut vector: Vec<Box<dyn std::any::Any>> = Vec::new();
    let streambox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    vector.push(Box::new(streambox.clone()));
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
    Stream.add_css_class("Stream");
    if probe != "error i chuj" {
        let find = ["artist          : ", "title           : ", "album           : ", "track           : ", "TRACKTOTAL      : ", "encoder         : ", "Duration: "];
        let nameBox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        streambox.append(&nameBox);
        match &exline(&probe, &find[1]) {
            Ok(res) => {
                let title = (&res[find[1].len()..]).to_string();
                vector.push(Box::new(title.clone()));
                let TitleLab = gtk::Label::builder()
                    .label(&title)
                    .build();
                nameBox.append(&TitleLab);
                TitleLab.add_css_class("title");
            },
            Err(_) => {
                let title = get_name(&*file);
                vector.push(Box::new(title.clone()));
                let TitleLab = gtk::Label::builder()
                    .label(&title)
                    .build();
                nameBox.append(&TitleLab);
                TitleLab.add_css_class("title");
            },
        }  
        match &exline(&probe, &find[0]) {
            Ok(res) => {
                let art = (&res[find[0].len()..]).to_string();
                vector.push(Box::new(art.clone()));
                let ArtistLab = gtk::Label::builder()
                    .label(&res[find[0].len()..])
                    .build();
                nameBox.append(&ArtistLab);
                ArtistLab.add_css_class("artist");
            }
            Err(_) => {
                vector.push(Box::new(""));
            },
        }  
        match &exline(&probe, &find[2]) {
            Ok(res) => {
                let alb = (&res[find[2].len()..]).to_string();
                vector.push(Box::new(alb.clone()));
                let AlbumLab = gtk::Label::builder()
                    .label(&res[find[2].len()..])
                    .build();
                //nameBox.append(&TitleLab);
                AlbumLab.add_css_class("album");
            },
            Err(_) => {
                vector.push(Box::new(""));
            },
        }  
        match &exline(&probe, &find[3]) {
            Ok(res) => {
                match &exline(&probe, &find[4]) {
                    Ok(ress) => {
                        let track = vec![res[find[3].len()..].parse::<i32>().unwrap(),ress[find[4].len()..].parse::<i32>().unwrap()];
                        vector.push(Box::new(track.clone()));
                        let TrackLab = gtk::Label::builder()
                            .label(format!("{}/{}", &track[0], &track[1]))
                            .build();
                        //nameBox.append(&TrackLab);
                        TrackLab.add_css_class("track");
                    },
                    Err(_) => {
                        let track = vec![res[find[3].len()..].parse::<i32>().unwrap(),-1];
                        vector.push(Box::new(track.clone()));
                        let TrackLab = gtk::Label::builder()
                            .label(format!("{}", &track[0]))
                            .build();
                        //nameBox.append(&TruckLab);
                        TrackLab.add_css_class("track");
                    },
                }  
            },
            Err(_) => {
                vector.push(Box::new(vec![-1,-1]));
            },
        }  
        match &exline(&probe, &find[5]) {
            Ok(res) => {
                let enco = (&res[find[5].len()..]).to_string();
                vector.push(Box::new(enco.clone()));
                let EncoLab = gtk::Label::builder()
                    .label(&res[find[5].len()..])
                    .build();
                //nameBox.append(&EncoLab);
                EncoLab.add_css_class("encoder");
            },
            Err(_) => {
                vector.push(Box::new(""));
            },
        }  
        match &exline(&probe, &find[6]) {
            Ok(res) => {
                let dur = (&res[find[6].len()..]).to_string();
                vector.push(Box::new(dur.clone()));
                let DurLab = gtk::Label::builder()
                    .label(&res[find[6].len()..])
                    .build();
                //nameBox.append(&DurLab);
                DurLab.add_css_class("duration");
            },
            Err(_) => {
                vector.push(Box::new(""));
            },
        }; 
    }
    vector
}
