pub mod imp;
mod bus;
mod pipeline;
mod actions;

use gstreamer::State;
use gtk::glib;
use gtk::prelude::ObjectExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;

#[derive(Clone)]
pub struct TrackRef {
    tab_idx: u32,
    track_idx: u32,
    filepath: String,
}

glib::wrapper! {
    pub struct BeatPlayer(ObjectSubclass<imp::BeatPlayerImp>);
}

impl Default for BeatPlayer {
    fn default() -> Self {
        glib::Object::new(&[])
    }
}

impl BeatPlayer {
    pub fn play_ref(&self, tab_idx: u32, track_idx: u32, filepath: String) {
        self.imp().play_ref(tab_idx, track_idx, filepath);
    }

    pub fn toggle_play(&self) {
        self.imp().play();
    }

    pub fn stop(&self) {
        self.imp().__stop();
    }

    pub fn next(&self) {
        self.imp().next();
    }

    pub fn prev(&self) {
        self.imp().prev();
    }

    pub fn add_to_queue(&self, tab_idx: u32, track_idx: u32, filepath: String) {

    }

    pub fn rm_from_queue(&self, tab_idx: u32, track_idx: u32) {

    }

    pub fn rm_tab_from_queue(&self, tab_idx: u32) {

    }

    pub fn seek_position(&self, percent: f64) {
        self.imp().__set_position_percent(percent);
    }

    pub fn set_volume(&self, value: f64) {
        self.imp().__set_volume(value);
    }

    pub fn destroy(&self) {
        self.imp().destroy();
    }

    fn __on_state_changed(&self, state: State) {
        self.emit_by_name::<()>("state-changed", &[&state]);
    }

    fn __on_stream_start(&self) {
        if let Some(duration) = self.imp().__get_duration() {
            self.emit_by_name::<()>("duration-changed", &[&duration]);
        }
    }

    fn __tick(&self) -> glib::Continue {
        if let Some((position, progress)) = self.imp().__get_position_progress() {
            self.emit_by_name::<()>("progress-changed", &[&position, &progress]);
        }
        glib::Continue(true)
    }

    fn __on_error(&self) {
        println!("error");
    }

    fn __on_eos(&self) {
        println!("error");
    }
}