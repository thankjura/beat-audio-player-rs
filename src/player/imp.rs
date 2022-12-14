use std::sync::Mutex;
use gstreamer::{Element, Pipeline, State};
use gstreamer::prelude::{ElementExtManual, GstBinExtManual};
use gstreamer_player::gst;
use gst::prelude::*;
use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::subclass::Signal;
use gtk::glib;
use gtk::subclass::prelude::*;
use crate::player::TrackRef;


pub struct BeatPlayerImp {
    pub pipeline: Pipeline,
    pub file_src: Element,
    pub volume: Element,
    decode_bin: Element,
    sink: Element,
    spectrum: Element,
    queue: Mutex<Vec<TrackRef>>,
    current_track: Mutex<Option<TrackRef>>
}


impl Default for BeatPlayerImp {
    fn default() -> Self {
        Self::new()
    }
}


impl BeatPlayerImp {
    pub fn new() -> Self {
        gst::init().unwrap();
        let pipeline = Pipeline::default();
        let file_src = gst::ElementFactory::make("filesrc").build().unwrap();
        let decode_bin = gst::ElementFactory::make("decodebin").build().unwrap();
        let audio_convert = gst::ElementFactory::make("audioconvert").build().unwrap();
        let volume = gst::ElementFactory::make("volume").build().unwrap();
        let sink = gst::ElementFactory::make("autoaudiosink").build().unwrap();
        let spectrum = gst::ElementFactory::make("spectrum").build().unwrap();
        spectrum.set_property("bands", 96u32);
        spectrum.set_property("threshold", -80);
        spectrum.set_property("interval", 50000000u64);
        spectrum.set_property("post-messages", true);
        spectrum.set_property("message-magnitude", true);

        let elements = [&file_src, &decode_bin, &audio_convert, &spectrum, &volume, &sink];
        pipeline.add_many(&elements).unwrap();
        file_src.link(&decode_bin).unwrap();
        audio_convert.link(&spectrum).unwrap();
        spectrum.link(&volume).unwrap();
        volume.link(&sink).unwrap();

        let sink_pad = audio_convert.static_pad("sink").unwrap();

        decode_bin.connect_pad_added(move |_, src_pad| {
            src_pad.link(&sink_pad).unwrap();
        });

        BeatPlayerImp {
            pipeline,
            file_src,
            volume,
            decode_bin,
            sink,
            spectrum,
            queue: Mutex::new(vec![]),
            current_track: Mutex::new(None),
        }
    }

    pub fn state(&self) -> Option<State> {
        if let (Ok(_val), state, _) = self.pipeline.state(None) {
            return Some(state);
        }

        None
    }

    pub fn set_current_track(&self, track_ref: TrackRef) -> Option<TrackRef> {
        self.obj().emit_by_name::<()>("track-changed", &[&track_ref.tab_idx, &track_ref.track_idx]);
        if let Some(old_ref) = self.current_track.lock().unwrap().replace(track_ref) {
            self.obj().emit_by_name::<()>("track-cleared", &[&old_ref.tab_idx, &old_ref.track_idx]);
            return Some(old_ref);
        }
        None
    }

    pub fn has_current_track(&self) -> bool {
        if let Some(_) = *self.current_track.lock().unwrap() {
            return true;
        }

        false
    }

    pub fn current_track(&self) -> Option<TrackRef> {
       let track = self.current_track.lock().unwrap();
       track.as_ref().cloned()
    }

    pub fn destroy(&self) {
        self.pipeline.set_state(State::Null).unwrap();
    }
}

#[glib::object_subclass]
impl ObjectSubclass for BeatPlayerImp {
    const NAME: &'static str = "BeatPlayer";
    type Type = super::BeatPlayer;
}

impl ObjectImpl for BeatPlayerImp {
    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![
                Signal::builder("state-changed")
                    .param_types([State::static_type()]).build(),
                Signal::builder("progress-changed")
                    .param_types([u64::static_type(), f64::static_type()]).build(),
                Signal::builder("duration-changed")
                    .param_types([u64::static_type()]).build(),
                Signal::builder("track-changed")
                    .param_types([u32::static_type(), u32::static_type()]).build(),
                Signal::builder("track-cleared")
                    .param_types([u32::static_type(), u32::static_type()]).build(),
            ]
        });

        SIGNALS.as_ref()
    }

    fn constructed(&self) {
        self.parent_constructed();
        self.watch_bus();
    }
}