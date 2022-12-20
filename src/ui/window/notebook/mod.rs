mod tab;
mod imp;
mod tabs;
mod playlist;

use rand::Rng;
use gstreamer::State;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
pub use tab::Tab;
use crate::BeatWindow;
use crate::structs::track::Track;

glib::wrapper! {
    pub struct BeatNotebook(ObjectSubclass<imp::BeatNotebookImp>)
        @extends gtk::Widget;
}

impl BeatNotebook {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::new(&[("application", app)])
    }

    pub fn tab_name(&self, tab_idx: u32) -> Option<String> {
        let tab_idx = usize::try_from(tab_idx).unwrap();

        if let Some(tab) = self.imp().tabs.borrow().get(tab_idx) {
            return Some(tab.label.label().as_str().to_string());
        }

        None
    }

    pub fn get_tracks(&self, tab_idx: u32) -> Vec<Track> {
        let tab_idx = usize::try_from(tab_idx).unwrap();
        if let Some(tab) = self.imp().tabs.borrow().get(tab_idx) {
            return tab.playlist().store().get_tracks();
        }
        vec![]
    }

    pub fn get_track(&self, tab_idx: u32, track_idx: u32) -> Option<Track> {
        let tab_idx = usize::try_from(tab_idx).unwrap();
        if let Some(tab) = self.imp().tabs.borrow().get(tab_idx) {
            return tab.playlist().store().get_track(track_idx);
        }

        None
    }

    pub fn set_track_state(&self, tab_idx: u32, track_idx: u32, state: Option<State>) {
        let tab_idx = usize::try_from(tab_idx).unwrap();
        if let Some(tab) = self.imp().tabs.borrow().get(tab_idx) {
            return tab.playlist().store().set_track_state(track_idx, state);
        }
    }


    pub fn set_track_position(&self, tab_idx: u32, track_idx: u32, position: u32) {
        let tab_idx = usize::try_from(tab_idx).unwrap();
        if let Some(tab) = self.imp().tabs.borrow().get(tab_idx) {
            return tab.playlist().store().set_track_position(track_idx, position);
        }
    }

    pub fn activate_next(&self) {
        let win = self.ancestor(BeatWindow::static_type()).unwrap();
        let win = win.downcast_ref::<BeatWindow>().unwrap();
        let shuffle = win.imp().button_shuffle.get().is_active();
        let repeat = win.imp().button_repeat.get().is_active();

        if let Some((tab_idx, tab, track_idx)) = self.imp().active_tab_track() {
            let mut next_index = None;
            if shuffle {
                let mut rng = rand::thread_rng();
                let random = rng.gen_range(0..tab.playlist().store().size());
                next_index = Some(random);
            } else {
                if let Some(_) = tab.playlist().store().get_track(track_idx + 1) {
                    next_index = Some(track_idx + 1);
                } else if repeat {
                    next_index = Some(0);
                }
            }

            if let Some(next_index) = next_index {
                if let Some(track) = tab.playlist().store().get_track(next_index) {
                    self.emit_by_name::<()>("track-activated", &[&tab_idx, &next_index, &track.filepath()]);
                }
            }
        }
    }

    pub fn activate_prev(&self) {
        if let Some((tab_idx, tab, track_idx)) = self.imp().active_tab_track() {
            if track_idx > 0 {
                if let Some(track) = tab.playlist().store().get_track(track_idx - 1) {
                    let index = track_idx - 1;
                    self.emit_by_name::<()>("track-activated", &[&tab_idx, &index, &track.filepath()]);
                }
            }
        }
    }
}