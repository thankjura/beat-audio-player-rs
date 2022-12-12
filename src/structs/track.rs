use std::cell::RefCell;
use lofty::{Accessor, AudioFile, ItemKey, Probe};
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum TrackState {
    Playing,
    Pause,
    Active,
    None
}

#[derive(Debug)]
pub struct Track {
    state: RefCell<TrackState>,
    filepath: String,
    filename: String,
    album: Option<String>,
    title: Option<String>,
    artist: Option<String>,
    year: Option<String>,
    duration: Option<u64>,
}

impl Clone for Track {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            filepath: self.filepath.to_string(),
            filename: self.filename.to_string(),
            album: self.album.clone(),
            title: self.title.clone(),
            artist: self.artist.clone(),
            year: self.year.clone(),
            duration: self.duration.clone()
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

                return Self {
                    state: RefCell::new(TrackState::None),
                    filepath: filepath.to_string(),
                    filename: filename.to_string(),
                    album: tag.get_string(&ItemKey::AlbumTitle).map(|s| s.to_string()),
                    title: tag.get_string(&ItemKey::TrackTitle).map(|s| s.to_string()),
                    artist: tag.get_string(&ItemKey::TrackArtist).map(|s| s.to_string()),
                    year: tag.year().map(|y| y.to_string()),
                    duration: Some(duration.as_secs())
                };
            };
        }


        Self {
            state: RefCell::new(TrackState::None),
            filepath: filepath.to_string(),
            filename: filename.to_string(),
            album: None,
            title: None,
            artist: None,
            year: None,
            duration: None
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

    pub fn set_state(&self, state: &TrackState) {
        self.state.replace(state.clone());
    }

    pub fn state(&self) -> TrackState {
        self.state.borrow().clone()
    }
}