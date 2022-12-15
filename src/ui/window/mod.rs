mod imp;
mod header;
mod notebook;
mod progress;
mod footer;
mod files;
mod spectrum;

use std::rc::Rc;
use gtk::{gio, glib};
use gtk::subclass::prelude::ObjectSubclassIsExt;
use crate::ui::window::notebook::Tab;

pub use notebook::BeatNotebook;

glib::wrapper! {
    pub struct BeatWindow(ObjectSubclass<imp::BeatWindowImp>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup, gtk::ConstraintTarget, gtk::Native, gtk::Root;
}

impl BeatWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::new(&[("application", app)])
    }

    pub fn selected_tab(&self) -> Rc<Tab> {
        self.imp().notebook.imp().selected_tab()
    }

    // pub fn get_track(&self, tab_idx: u32, track_idx: u32) -> Option<Track> {
    //     self.imp().notebook.get_track(tab_idx, track_idx)
    // }
    //
    // pub fn set_track_state(&self, tab_idx: u32, track_idx: u32, state: &TrackState) {
    //     self.imp().notebook.set_track_state(tab_idx, track_idx, state);
    //
    //     if let Some(track) = self.get_track(tab_idx, track_idx) {
    //
    //     }
    // }
}

