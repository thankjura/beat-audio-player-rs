mod queue;

use gtk::glib;
use gtk::subclass::prelude::*;


glib::wrapper! {
    pub struct Queue(ObjectSubclass<queue::QueueImp>);
}


pub struct TrackRef {
    tab_idx: u32,
    track_idx: u32,
}


impl Queue {
    pub fn add_to_queue(&self, tab_idx: u32, track_idx: u32) {
        self.imp().queue.borrow_mut().push(TrackRef { tab_idx, track_idx });
    }

    pub fn remove_from_queue(&self, tab_idx: u32, track_idx: u32) {
        let obj = self.imp();

        if let Some(position) = obj.queue.borrow().iter().position(|t| {
            t.tab_idx == tab_idx && t.track_idx == track_idx
        }) {
            obj.queue.borrow_mut().remove(position);
            for tr in &obj.queue.borrow()[0..] {
                // TODO: recalc playlist
            }
        }
    }
}