use std::cell::Ref;
use gtk::{glib, gio};
use gtk::glib::BoxedAnyObject;
use gtk::prelude::*;
use crate::ui::window::notebook::playlist::track::Track;


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

    pub fn add_row(&self, track: Track) {
        self.store.append(&glib::BoxedAnyObject::new(track));
    }

    pub fn get_row(&self, index: u32) -> Option<Track> {
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
}

impl Default for PlayListStore {
    fn default() -> Self {
        Self::new()
    }
}