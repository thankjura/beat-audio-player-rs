use std::cell::Ref;
use gtk::{glib, gio};
use gtk::glib::BoxedAnyObject;
use gtk::prelude::*;
use crate::structs::track::{Track, TrackState};


#[derive(Debug)]
pub struct PlayListStore {
    store: gio::ListStore,
    selector: gtk::SingleSelection,
}

impl PlayListStore {
    pub fn new() -> Self {
        let store = gio::ListStore::new(glib::BoxedAnyObject::static_type());
        let selector = gtk::SingleSelection::new(Some(&store));

        Self {
            store,
            selector
        }
    }

    pub fn selector(&self) -> &gtk::SingleSelection {
        &self.selector
    }

    pub fn add_track(&self, track: Track) {
        self.store.append(&glib::BoxedAnyObject::new(track));
    }

    pub fn get_track(&self, index: u32) -> Option<Track> {
        if let Some(item) = self.selector.model().unwrap().item(index) {
            let entry = item.downcast::<BoxedAnyObject>().unwrap();
            let r: Ref<Track> = entry.borrow();
            return Some(r.clone());
        }

        None
    }

    pub fn has_tracks(&self) -> bool {
        self.selector.n_items() > 0
    }

    pub fn clear(&self) {
        self.store.remove_all();
    }

    pub fn set_track_state(&self, index: u32, state: &TrackState) {
        if let Some(item) = self.selector.model().unwrap().item(index) {
            let entry = item.downcast::<BoxedAnyObject>().unwrap();
            let r: Ref<Track> = entry.borrow();
            println!("{:#?}", &r);
            r.set_state(state);
            self.store.items_changed(index, 0, 0);
        }
    }
}

impl Default for PlayListStore {
    fn default() -> Self {
        Self::new()
    }
}