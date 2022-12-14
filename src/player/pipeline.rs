use gstreamer::prelude::*;
use gstreamer::State;
use gstreamer_player::gst;
use gtk::subclass::prelude::*;
use crate::player::imp::BeatPlayerImp;

impl BeatPlayerImp {
    pub fn __set_uri(&self, uri: &str) {
        self.__stop();
        self.file_src.set_property_from_str("location", uri);
    }

    pub fn __set_state(&self, state: State) {
        match self.pipeline.set_state(state) {
            Ok(_) => {
                self.obj().emit_by_name::<()>("state-changed", &[&state]);
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

    pub fn __set_position_percent(&self, progress: f64) {
        if let Some(duration) = self.__get_duration() {
            let seek_value = ((duration as f64 / 100.0) * progress) as u64;
            if let Err(_) = self.pipeline.seek_simple(gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT,  seek_value * gst::ClockTime::SECOND) {
                println!("Can't seek");
            } else {
                self.obj().emit_by_name::<()>("progress-changed", &[&seek_value, &progress]);
            }
        }
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