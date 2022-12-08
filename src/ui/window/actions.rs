use gtk::subclass::prelude::*;
use std::path::Path;
use gtk::gio::SimpleAction;
use gtk::prelude::*;
use crate::ui::window::notebook::{Track, TrackRef};

impl super::BeatWindow {
    pub fn open_path(&self, path: &str) {
        let path = Path::new(path);
        if path.is_file() {
            let tab = self.imp().notebook.imp().selected_tab();
            tab.add_track(Track::new(path));
        }
    }

    pub fn setup_actions(&self) {
        let action = SimpleAction::new("track_activate", Some(&TrackRef::static_variant_type()));
        let q = self.imp().queue_manager.clone();
        action.connect_activate(move |_action, parameter| {
            let track_ref = parameter.expect("No track received").get::<TrackRef>().expect("Not is TrackRef format");
            q.play(track_ref);
        });
        self.add_action(&action);
    }
}