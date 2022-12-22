use std::cell::Ref;
use gstreamer::State;
use gtk::{glib, gio};
use gtk::glib::BoxedAnyObject;
use gtk::prelude::*;
use crate::structs::track::Track;


#[derive(Debug)]
pub struct PlayListStore {
    store: gio::ListStore,
    selector: gtk::MultiSelection,
}

pub fn get_track(store: &gio::ListStore, index: u32) -> Option<Track> {
    if let Some(item) = store.item(index) {
        let entry = item.downcast::<BoxedAnyObject>().unwrap();
        let r: Ref<Track> = entry.borrow();
        return Some(r.clone());
    }

    None
}

impl PlayListStore {
    pub fn new() -> Self {
        let store = gio::ListStore::new(glib::BoxedAnyObject::static_type());
        let selector = gtk::MultiSelection::new(Some(&store));

        Self {
            store,
            selector
        }
    }

    pub fn selector(&self) -> &gtk::MultiSelection {
        &self.selector
    }

    pub fn store(&self) -> &gio::ListStore {
        &self.store
    }

    pub fn add_track(&self, track: Track) {
        self.store.append(&glib::BoxedAnyObject::new(track));
    }

    pub fn get_track(&self, index: u32) -> Option<Track> {
        get_track(&self.store, index)
    }

    pub fn get_tracks(&self) -> Vec<Track> {
        let mut out = vec![];

        for i in 0..self.store.n_items() {
            if let Some(track) = get_track(&self.store, i) {
                out.push(track);
            }
        }

        out
    }


    pub fn rm_track(&self, index: u32) {
        self.store.remove(index);
    }

    pub fn has_tracks(&self) -> bool {
        self.store.n_items() > 0
    }

    pub fn active_track(&self) -> Option<u32> {
        for i in 0..self.store.n_items() {
            if let Some(track) = get_track(&self.store, i) {
                if track.state().is_some() {
                    return Some(i);
                }
            }
        }

        None
    }

    pub fn clear(&self) {
        self.store.remove_all();
    }

    pub fn set_track_state(&self, index: u32, state: Option<State>) {
        if let Some(item) = self.selector.model().unwrap().item(index) {
            let entry = item.downcast::<BoxedAnyObject>().unwrap();
            let r: Ref<Track> = entry.borrow();
            r.set_state(state);
            self.store.items_changed(index, 0, 0);
        }
    }

    pub fn set_track_duration(&self, index: u32, duration: u64) {
        if let Some(item) = self.selector.model().unwrap().item(index) {
            let entry = item.downcast::<BoxedAnyObject>().unwrap();
            let r: Ref<Track> = entry.borrow();
            r.set_duration(duration);
            self.store.items_changed(index, 0, 0);
        }
    }

    pub fn set_track_position(&self, index: u32, position: u32) {
        if let Some(item) = self.selector.model().unwrap().item(index) {
            let entry = item.downcast::<BoxedAnyObject>().unwrap();
            let r: Ref<Track> = entry.borrow();
            r.set_queue_pos(position);
            self.store.items_changed(index, 0, 0);
        }
    }

    pub fn size(&self) -> u32 {
        self.store.n_items()
    }
}

impl Default for PlayListStore {
    fn default() -> Self {
        Self::new()
    }
}