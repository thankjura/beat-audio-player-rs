use gettextrs::gettext;
use crate::gio::subclass::prelude::*;
use crate::ui::window::imp::BeatWindowImp;
use crate::utils::meta;


impl BeatWindowImp {
    pub fn open_path(&self, paths: Vec<String>, append: bool) {
        let mut tab = self.notebook.imp().selected_tab();
        if !append && tab.has_tracks() {
            tab = self.notebook.imp().add_tab(&gettext("new"));
        }

        for path in paths {
            if let Some(track) = meta::get_track_from_path(&path) {
                tab.add_track(track);
            }
        }
    }
}