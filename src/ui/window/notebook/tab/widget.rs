use gtk::ScrolledWindow;
use crate::ui::window::notebook::playlist::{PlayList, Track};

#[derive(Debug, Default)]
pub struct Tab {
    label: gtk::Label,
    playlist: PlayList
}

impl Tab {
    pub fn new(name: &str) -> Self {
        let label = gtk::Label::new(Some(name));
        let playlist = PlayList::new_with_uuid(&uuid::Uuid::new_v4().to_string());

        Self {
            label,
            playlist
        }
    }

    pub fn label(&self) -> &gtk::Label {
        &self.label
    }

    pub fn scrollbox(&self) -> &ScrolledWindow {
        self.playlist.scrollbox()
    }

    pub fn add_track(&self, track: Track) {
        self.playlist.store().add_row(track);
    }
}
