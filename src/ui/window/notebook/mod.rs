mod tab;
mod imp;
mod tabs;
mod playlist;

use gtk::glib;
use gtk::subclass::prelude::*;
pub use playlist::Track;
pub use tab::Tab;

glib::wrapper! {
    pub struct BeatNotebook(ObjectSubclass<imp::BeatNotebookImp>)
        @extends gtk::Widget;
}

impl BeatNotebook {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::new(&[("application", app)])
    }

    pub fn get_track(&self, tab_idx: u32, track_idx: u32) -> Option<Track> {
        let tab_idx = usize::try_from(tab_idx).unwrap();
        if let Some(tab) = self.imp().tabs.borrow().get(tab_idx) {
            return tab.playlist().store().get_row(track_idx);
        }

        None
    }
}