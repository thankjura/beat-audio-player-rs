use std::cell::RefCell;
use gstreamer::State;

#[derive(Debug)]
pub struct Track {
    state: RefCell<Option<State>>,
    filepath: String,
    filename: String,
    album: Option<String>,
    title: Option<String>,
    artist: Option<String>,
    year: Option<String>,
    duration: Option<u64>,
    queue_position: Option<String>,
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
            duration: self.duration.clone(),
            queue_position: None
        }
    }
}

impl Track {
    pub fn new(filename: &str, filepath: &str, album: Option<&str>, title: Option<&str>, artist: Option<&str>, year: Option<u32>, duration: Option<u64>) -> Track {
        Self {
            state: RefCell::new(None),
            filepath: filepath.to_string(),
            filename: filename.to_string(),
            album: album.map(|s| s.to_string()),
            title: title.map(|s| s.to_string()),
            artist: artist.map(|s| s.to_string()),
            year: year.map(|y| y.to_string()),
            duration,
            queue_position: None
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
            "position" => {
                self.queue_position.as_deref()
            },
            _ => {
                None
            }
        }
    }

    pub fn filepath(&self) -> &str {
        &self.filepath
    }

    pub fn set_state(&self, state: Option<State>) {
        self.state.replace(state);
    }

    pub fn set_queue_pos(&mut self, position: String) {
        self.queue_position.replace(position);
    }

    pub fn state(&self) -> Option<State> {
        self.state.borrow().clone()
    }
}