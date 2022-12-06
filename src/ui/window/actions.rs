use gtk::subclass::prelude::*;
use std::path::Path;
use crate::ui::window::notebook::Track;

impl super::BeatWindow {
    pub fn open_path(&self, path: &str) {
        let path = Path::new(path);
        if path.is_file() {
            let tab = self.imp().notebook.imp().selected_tab();
            tab.add_track(Track::new(path));
        }
    }
}