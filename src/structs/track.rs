use std::cell::RefCell;
use gstreamer::State;

#[derive(Debug)]
pub struct Track {
    state: RefCell<State>,
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
    pub fn new(filename: &str, filepath: &str, album: Option<&str>, title: Option<&str>, artist: Option<&str>, year: Option<u32>, duration: Option<u64>) -> Track {
        Self {
            state: RefCell::new(State::Null),
            filepath: filepath.to_string(),
            filename: filename.to_string(),
            album: album.map(|s| s.to_string()),
            title: title.map(|s| s.to_string()),
            artist: artist.map(|s| s.to_string()),
            year: year.map(|y| y.to_string()),
            duration,
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

    pub fn filepath(&self) -> &str {
        &self.filepath
    }

    pub fn set_state(&self, state: &State) {
        self.state.replace(state.clone());
    }

    pub fn state(&self) -> State {
        self.state.borrow().clone()
    }
}