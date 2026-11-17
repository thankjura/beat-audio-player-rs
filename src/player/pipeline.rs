use crate::player::imp::BeatPlayerImp;
use gstreamer as gst;
use gstreamer::prelude::*;
use gstreamer::State;
use gtk::glib;
use gtk::subclass::prelude::*;
use std::time::Duration;

impl BeatPlayerImp {
    pub fn __set_uri(&self, uri: &str) {
        self.__stop();
        self.file_src.set_property_from_str("location", uri);
    }

    pub fn __set_state(&self, state: State) {
        match self.pipeline.set_state(state) {
            Ok(_) => {
                self.obj().__on_state_changed(state);
            }
            Err(_) => {
                println!("Failed to change state");
            }
        }
    }

    pub fn __stop(&self) {
        self.__set_state(State::Null);
    }

    pub fn __play(&self) {
        self.__set_state(State::Playing);
    }

    pub fn __pause(&self) {
        self.__set_state(State::Paused);
    }

    pub fn __toggle_play(&self) {
        if let Some(State::Playing) = self.state() {
            self.__pause();
        } else {
            self.__play();
        }
    }

    pub fn __set_volume(&self, value: f64) {
        self.volume.set_property("volume", value);
    }

    pub fn set_position_percent_smooth(&self, progress: f64) {
        if let Some(timer_id) = self.seek_timeout.lock().unwrap().take() {
            timer_id.remove();
        }

        let player_ref = self.downgrade();
        let timer = glib::timeout_add_once(Duration::from_millis(300), move || {
            if let Some(player) = player_ref.upgrade() {
                player.seek_timeout.lock().unwrap().take();
                player.__set_position_percent(progress);
            }
        });

        self.seek_timeout.lock().unwrap().replace(timer);
    }

    pub fn __set_position_percent(&self, progress: f64) {
        if let Some(duration) = self.__get_duration() {
            self.seek_to(((duration as f64 / 100.0) * progress) as u64);
        }
    }

    /// Seeks to `seconds` from the start of the current track and notifies
    /// listeners through `progress-changed` and `seeked`.
    pub fn seek_to(&self, seconds: u64) {
        let Some(duration) = self.__get_duration() else {
            return;
        };
        let seconds = seconds.min(duration);
        if self
            .pipeline
            .seek_simple(
                gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT,
                seconds * gst::ClockTime::SECOND,
            )
            .is_err()
        {
            println!("Can't seek");
            return;
        }

        let progress = if duration > 0 {
            seconds as f64 / duration as f64 * 100.0
        } else {
            0.0
        };
        self.obj()
            .emit_by_name::<()>("progress-changed", &[&seconds, &progress]);
        self.obj().emit_by_name::<()>("seeked", &[&seconds]);
    }

    pub fn __get_position_progress(&self) -> Option<(u64, f64)> {
        if let Some(position) = self.pipeline.query_position::<gst::ClockTime>() {
            if let Some(duration) = self.__get_duration() {
                let progress = (position.seconds() as f64 / duration as f64) * 100.0;
                return Some((position.seconds(), progress));
            }
        }
        None
    }

    pub fn __get_duration(&self) -> Option<u64> {
        if let Some(duration) = self.pipeline.query_duration::<gst::ClockTime>() {
            return Some(duration.seconds());
        }
        None
    }
}
