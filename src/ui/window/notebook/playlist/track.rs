use lofty::{Accessor, AudioFile, ItemKey, Probe};
use std::path::Path;


#[derive(Debug)]
pub struct Track {
    filepath: String,
    filename: String,
    album: Option<String>,
    title: Option<String>,
    artist: Option<String>,
    year: Option<String>,
    duration: Option<u64>,
    duration_str: Option<String>,
}

impl Clone for Track {
    fn clone(&self) -> Self {
        Self {
            filepath: self.filepath.to_string(),
            filename: self.filename.to_string(),
            album: self.album.clone(),
            title: self.title.clone(),
            artist: self.artist.clone(),
            year: self.year.clone(),
            duration: self.duration.clone(),
            duration_str: self.duration_str.clone(),
        }
    }
}

impl Track {
    pub fn new(file: &Path) -> Self {
        let filepath = &file.to_str().unwrap().to_string();
        let filename = &file.file_name().unwrap().to_str().unwrap().to_string();

        if let Ok(tagged_file) = Probe::open(file).unwrap().read() {
            let tag = match tagged_file.primary_tag() {
                Some(primary_tag) => Some(primary_tag),
                None => tagged_file.first_tag(),
            };

            if let Some(tag) = tag {
                let properties = tagged_file.properties();
                let duration = properties.duration();
                let seconds = duration.as_secs() % 60;

                return Self {
                    filepath: filepath.to_string(),
                    filename: filename.to_string(),
                    album: tag.get_string(&ItemKey::AlbumTitle).map(|s| s.to_string()),
                    title: tag.get_string(&ItemKey::TrackTitle).map(|s| s.to_string()),
                    artist: tag.get_string(&ItemKey::TrackArtist).map(|s| s.to_string()),
                    year: tag.year().map(|y| y.to_string()),
                    duration: Some(duration.as_secs()),
                    duration_str: Some(format!("{}:{:02}", (duration.as_secs() - seconds) / 60, seconds)),
                };
            };
        }


        Self {
            filepath: filepath.to_string(),
            filename: filename.to_string(),
            album: None,
            title: None,
            artist: None,
            year: None,
            duration: None,
            duration_str: None,
        }
    }

    pub fn get_by_name(&self, field: &str) -> Option<&str> {
        match field {
            "filename" => {
                Some(&self.filename)
            },
            "filepath" => {
                Some(&self.filepath)
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

    // pub fn set_duration(&mut self, duration: u32) {
    //     let (minutes, seconds) = &duration.div_rem(&60);
    //
    //     self.duration_str.replace(format!("{}:{:02}", minutes, seconds));
    //     self.duration.replace(duration);
    // }

    pub fn filepath(&self) -> &str {
        &self.filepath
    }
}