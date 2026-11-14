use crate::structs::track::Track;
use gstreamer::State;
use gtk::glib::BoxedAnyObject;
use gtk::prelude::*;
use gtk::{gio, glib};
use std::cell::Ref;

#[derive(Clone)]
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
        let store = gio::ListStore::new::<BoxedAnyObject>();
        let selector = gtk::MultiSelection::new(Some(store.clone()));

        Self { store, selector }
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

    fn update_track(&self, index: u32, update: impl FnOnce(&Track)) {
        let Some(track) = self.get_track(index) else {
            return;
        };
        update(&track);

        // BoxedAnyObject cannot notify GTK about changes inside Track. Replace the
        // wrapper so ColumnView sees a new item and binds the row again.
        let was_selected = self.selector.is_selected(index);
        self.store.splice(index, 1, &[BoxedAnyObject::new(track)]);
        if was_selected {
            self.selector.select_item(index, true);
        }
    }

    pub fn set_track_state(&self, index: u32, state: Option<State>) {
        self.update_track(index, |track| track.set_state(state));
    }

    pub fn set_track_duration(&self, index: u32, duration: u64) {
        self.update_track(index, |track| track.set_duration(duration));
    }

    pub fn set_track_position(&self, index: u32, position: u32) {
        self.update_track(index, |track| track.set_queue_pos(position));
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

#[cfg(test)]
mod tests {
    use super::PlayListStore;
    use crate::structs::track::Track;
    use gtk::prelude::*;
    use std::sync::mpsc;

    #[test]
    fn queue_position_is_updated_and_cleared() {
        gtk::init().expect("GTK must be available for the playlist test");
        let store = PlayListStore::new();
        store.add_track(Track::new(
            "track.mp3",
            "/tmp/track.mp3",
            None,
            None,
            None,
            None,
            None,
        ));

        let (changed_tx, changed_rx) = mpsc::channel();
        store
            .store()
            .connect_items_changed(move |_, position, removed, added| {
                changed_tx.send((position, removed, added)).unwrap();
            });
        assert!(store.selector().select_item(0, true));
        let previous_item = store.store().item(0).unwrap();

        store.set_track_position(0, 1);
        assert_eq!(
            store.get_track(0).unwrap().queue_pos().as_deref(),
            Some("1")
        );
        assert_eq!(changed_rx.recv().unwrap(), (0, 1, 1));
        assert_ne!(store.store().item(0).unwrap(), previous_item);
        assert!(store.selector().is_selected(0));

        store.set_track_position(0, 0);
        assert_eq!(store.get_track(0).unwrap().queue_pos(), None);
        assert_eq!(changed_rx.recv().unwrap(), (0, 1, 1));
        assert!(store.selector().is_selected(0));
    }
}
