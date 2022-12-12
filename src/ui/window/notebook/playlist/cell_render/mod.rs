mod widget;

use gtk::glib;
use gtk::subclass::prelude::*;
use crate::structs::track::TrackState;

glib::wrapper! {
    pub struct CellTrackState(ObjectSubclass<widget::CellTrackStateImp>)
        @extends gtk::Widget;
}

// impl Default for CellTrackState {
//     fn default() -> Self {
//         Self::new()
//     }
// }
//
// //
// // pub struct Entry {
// //     pub name: String,
// // }
// //
impl CellTrackState {
    pub fn set_state(&self, state: &TrackState) {
        self.imp().set_state(state);
    }
}