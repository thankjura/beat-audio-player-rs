use std::cell::{Cell, RefCell};

use gtk::glib;
use gtk::subclass::prelude::*;
use crate::structs::queue::TrackRef;

#[derive(Default)]
pub struct QueueImp {
    pub queue: RefCell<Vec<TrackRef>>,
    pub current_track: Cell<Option<TrackRef>>
}

#[glib::object_subclass]
impl ObjectSubclass for QueueImp {
    const NAME: &'static str = "Queue";
    type Type = super::Queue;
}

impl ObjectImpl for QueueImp {}