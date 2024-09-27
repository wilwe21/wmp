use gtk::prelude::*;

#[derive(Debug)]
pub struct Music {
    pub streambox: gtk::Box,
    pub file: String,
    pub title: String,
    pub author: String,
    pub album: String,
    pub track: u8,
    pub totaltrack: u8,
    pub encoder: String,
}
impl Music {
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

