mod tab;
mod imp;
mod tabs;
mod playlist;

use gtk::glib;
use gtk::subclass::prelude::*;
pub use tab::Tab;
use crate::structs::track::{Track, TrackState};

glib::wrapper! {
    pub struct BeatNotebook(ObjectSubclass<imp::BeatNotebookImp>)
        @extends gtk::Widget;
}

impl BeatNotebook {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::new(&[("application", app)])
    }

    pub fn get_track(&self, tab_idx: u32, track_idx: u32) -> Option<Track> {
        let tab_idx = usize::try_from(tab_idx).unwrap();
        if let Some(tab) = self.imp().tabs.borrow().get(tab_idx) {
            return tab.playlist().store().get_track(track_idx);
        }

        None
    }

    pub fn set_track_state(&self, tab_idx: u32, track_idx: u32, state: &TrackState) {
        let tab_idx = usize::try_from(tab_idx).unwrap();
        if let Some(tab) = self.imp().tabs.borrow().get(tab_idx) {
            return tab.playlist().store().set_track_state(track_idx, state);
        }
    }
}