use std::rc::Rc;
use gtk::glib;
use gtk::glib::{Receiver, SignalHandlerId};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use crate::app::imp::BeatAppImp;
use crate::BeatWindow;
use crate::structs::track::TrackState;

pub enum AppMessage {
    ProgressChanged(u64, f64),
    DurationChanged(u64),
    PlayingState(bool),
    ClearProgress,
    TrackChanged(u32, u32, TrackState)
}

impl BeatAppImp {
    pub fn watch_channel(&self, receiver: Receiver<AppMessage>, window: &Rc<BeatWindow>, progress_value_signal: SignalHandlerId) {
        let window_ref = window.downgrade();

        receiver.attach(None, move |value: AppMessage| {
            let window = window_ref.upgrade().unwrap();
            window.imp().progress.block_signal(&progress_value_signal);
            match value {
                AppMessage::ProgressChanged(position, progress) => {
                    window.imp().update_progress(position, progress);
                }
                AppMessage::DurationChanged(value) => {
                    window.imp().update_duration(value);
                }
                AppMessage::PlayingState(value) => {
                    window.imp().set_playing_icon(value);
                }
                AppMessage::ClearProgress => {
                    window.imp().clear_duration();
                }
                AppMessage::TrackChanged(tab_idx, track_idx, state) => {
                    window.imp().notebook.get().set_track_state(tab_idx, track_idx, &state);
                }
            }
            window.imp().progress.unblock_signal(&progress_value_signal);
            glib::Continue(true)
        });
    }
}