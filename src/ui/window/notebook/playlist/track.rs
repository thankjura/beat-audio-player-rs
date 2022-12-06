use std::path::Path;
use audiotags::Tag;
use num_integer::Integer;


#[derive(Debug)]
pub struct Track {
    filename: String,
    path: String,
    album: Option<String>,
    title: Option<String>,
    artist: Option<String>,
    year: Option<String>,
    duration: Option<u32>,
    duration_str: Option<String>,

}

impl Track {
    pub fn new(file: &Path) -> Self {
        if let Ok(tag) = Tag::new().read_from_path(file) {
            Self {
                path: file.to_str().unwrap().to_string(),
                filename: file.file_name().unwrap().to_str().unwrap().to_string(),
                album: tag.album_title().map(|s| s.to_string()),
                title: tag.title().map(|s| s.to_string()),
                artist: tag.artist().map(|s| s.to_string()),
                year: tag.year().map(|y| y.to_string()),
                duration: None,
                duration_str: None
            }
        } else {
            Self {
                path: file.to_str().unwrap().to_string(),
                filename: file.file_name().unwrap().to_str().unwrap().to_string(),
                album: None,
                title: None,
                artist: None,
                year: None,
                duration: None,
                duration_str: None
            }
        }
    }

    pub fn get_by_name(&self, field: &str) -> Option<&str> {
        match field {
            "filename" => {
                Some(&self.filename)
            },
            "path" => {
                Some(&self.path)
            },
            "album" => {
                self.album.as_deref()
            },
            "title" => {
                self.title.as_deref()
            },
            "artist" => {
                self.artist.as_deref()
            },
            "duration" => {
                self.duration_str.as_deref()
            },
            _ => {
                None
            }
        }
    }

    pub fn set_duration(&mut self, duration: u32) {
        let (minutes, seconds) = &duration.div_rem(&60);

        self.duration_str.replace(format!("{}:{:02}", minutes, seconds));
        self.duration.replace(duration);
    }
}