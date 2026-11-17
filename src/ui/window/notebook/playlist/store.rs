use crate::structs::track::Track;
use crate::ui::window::notebook::playlist::item::TrackItem;
use gstreamer::State;
use gtk::gio;
use gtk::prelude::*;

#[derive(Clone)]
pub struct PlayListStore {
    store: gio::ListStore,
    selector: gtk::MultiSelection,
}

pub fn get_track(store: &gio::ListStore, index: u32) -> Option<Track> {
    track_item(store, index).map(|item| item.track().clone())
}

fn track_item(store: &gio::ListStore, index: u32) -> Option<TrackItem> {
    store.item(index).and_downcast::<TrackItem>()
}

impl PlayListStore {
    pub fn new() -> Self {
        let store = gio::ListStore::new::<TrackItem>();
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
        self.store.append(&TrackItem::new(track));
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

    // Update the item in place: replacing it in the store would make the
    // ColumnView drop the focused row and jump back to the top.
    pub fn set_track_state(&self, index: u32, state: Option<State>) {
        if let Some(item) = track_item(&self.store, index) {
            item.set_state(state);
        }
    }

    pub fn set_track_duration(&self, index: u32, duration: u64) {
        if let Some(item) = track_item(&self.store, index) {
            item.set_duration(duration);
        }
    }

    pub fn set_track_position(&self, index: u32, position: u32) {
        if let Some(item) = track_item(&self.store, index) {
            item.set_queue_pos(position);
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

#[cfg(test)]
mod tests {
    use super::PlayListStore;
    use crate::structs::track::Track;
    use gstreamer::State;
    use gtk::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn track_updates_keep_item_and_notify_properties() {
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

        let items_changed = Rc::new(RefCell::new(0));
        let counter = items_changed.clone();
        store
            .store()
            .connect_items_changed(move |_, _, _, _| *counter.borrow_mut() += 1);

        let item = store.store().item(0).unwrap();
        let notified = Rc::new(RefCell::new(Vec::<String>::new()));
        let log = notified.clone();
        item.connect_notify_local(None, move |_, pspec| {
            log.borrow_mut().push(pspec.name().to_string())
        });
        assert!(store.selector().select_item(0, true));

        store.set_track_position(0, 1);
        assert_eq!(
            store.get_track(0).unwrap().queue_pos().as_deref(),
            Some("1")
        );
        assert_eq!(
            item.property::<Option<String>>("queue-pos").as_deref(),
            Some("1")
        );

        store.set_track_state(0, Some(State::Playing));
        assert_eq!(
            item.property::<Option<String>>("icon-name").as_deref(),
            Some("beat-play-symbolic")
        );

        store.set_track_position(0, 0);
        assert_eq!(store.get_track(0).unwrap().queue_pos(), None);
        assert_eq!(item.property::<Option<String>>("queue-pos"), None);

        assert_eq!(*notified.borrow(), ["queue-pos", "icon-name", "queue-pos"]);
        // The row object is never replaced, so the view keeps focus and scroll.
        assert_eq!(*items_changed.borrow(), 0);
        assert_eq!(store.store().item(0).unwrap(), item);
        assert!(store.selector().is_selected(0));
    }
}
