use crate::utils::format::time_str;
use gstreamer::State;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Track {
    state: RefCell<Option<State>>,
    filepath: String,
    filename: String,
    album: Option<String>,
    title: Option<String>,
    artist: Option<String>,
    year: Option<String>,
    duration_str: RefCell<Option<String>>,
    queue_position: RefCell<Option<String>>,
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
            duration_str: self.duration_str.clone(),
            queue_position: self.queue_position.clone(),
        }
    }
}

impl Track {
    pub fn new(
        filename: &str,
        filepath: &str,
        album: Option<&str>,
        title: Option<&str>,
        artist: Option<&str>,
        year: Option<u32>,
        duration_str: Option<&str>,
    ) -> Track {
        Self {
            state: RefCell::new(None),
            filepath: filepath.to_string(),
            filename: filename.to_string(),
            album: album.map(|s| s.to_string()),
            title: title.map(|s| s.to_string()),
            artist: artist.map(|s| s.to_string()),
            year: year.map(|y| y.to_string()),
            duration_str: RefCell::new(duration_str.map(|s| s.to_string())),
            queue_position: RefCell::new(None),
        }
    }

    pub fn get_by_name(&self, field: &str) -> Option<&str> {
        match field {
            "filename" => Some(&self.filename),
            "filepath" => Some(&self.filepath),
            "album" => self.album.as_deref(),
            "title" => self.title.as_deref(),
            "artist" => self.artist.as_deref(),
            _ => None,
        }
    }

    pub fn filepath(&self) -> &str {
        &self.filepath
    }

    pub fn set_state(&self, state: Option<State>) {
        self.state.replace(state);
    }

    pub fn set_duration(&self, duration: u64) {
        self.duration_str.replace(Some(time_str(duration)));
    }

    pub fn duration(&self) -> Option<String> {
        self.duration_str.borrow().clone()
    }

    pub fn set_queue_pos(&self, position: u32) {
        if position > 0 {
            self.queue_position.replace(Some(position.to_string()));
        } else {
            self.queue_position.replace(None);
        }
    }

    pub fn queue_pos(&self) -> Option<String> {
        self.queue_position.borrow().clone()
    }

    pub fn state(&self) -> Option<State> {
        *self.state.borrow()
    }
}
