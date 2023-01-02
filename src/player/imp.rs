use crate::player::TrackRef;
use crate::structs::{SPECTRUM_BANDS, SPECTRUM_INTERVAL, SPECTRUM_THRESHOLD};
use gst::prelude::*;
use gstreamer::prelude::{ElementExtManual, GstBinExtManual};
use gstreamer::{Element, Pipeline, State};
use gstreamer_player::gst;
use gtk::glib;
use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::subclass::Signal;
use gtk::subclass::prelude::*;
use std::collections::VecDeque;
use std::sync::Mutex;

pub struct BeatPlayerImp {
    pub pipeline: Pipeline,
    pub file_src: Element,
    pub volume: Element,
    pub queue: Mutex<VecDeque<TrackRef>>,
    current_track: Mutex<Option<TrackRef>>,
    pub seek_timeout: Mutex<Option<glib::SourceId>>,
    //next_cb: Option<Box<dyn Fn() -> (u32, u32, String) + Send + Sync + 'static>>
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
        spectrum.set_property("bands", SPECTRUM_BANDS);
        spectrum.set_property("threshold", SPECTRUM_THRESHOLD);
        spectrum.set_property("interval", SPECTRUM_INTERVAL);
        spectrum.set_property("post-messages", true);
        spectrum.set_property("message-magnitude", true);

        let elements = [
            &file_src,
            &decode_bin,
            &audio_convert,
            &spectrum,
            &volume,
            &sink,
        ];
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
            queue: Mutex::new(VecDeque::new()),
            current_track: Mutex::new(None),
            seek_timeout: Mutex::new(None),
            //next_cb: None,
        }
    }

    pub fn state(&self) -> Option<State> {
        if let (Ok(_val), state, _) = self.pipeline.state(None) {
            return Some(state);
        }

        None
    }

    pub fn set_current_track(&self, track_ref: TrackRef) -> Option<TrackRef> {
        if let Some(old_ref) = self.current_track.lock().unwrap().replace(track_ref) {
            self.obj()
                .emit_by_name::<()>("track-cleared", &[&old_ref.tab_idx, &old_ref.track_idx]);
            self.obj().emit_by_name::<()>(
                "queue-changed",
                &[&old_ref.tab_idx, &old_ref.track_idx, &0u32],
            );
            return Some(old_ref);
        }
        None
    }

    pub fn has_current_track(&self) -> bool {
        if self.current_track.lock().unwrap().is_some() {
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
                    .param_types([
                        i32::static_type(),
                        i32::static_type(),
                        String::static_type(),
                        State::static_type(),
                    ])
                    .build(),
                Signal::builder("progress-changed")
                    .param_types([u64::static_type(), f64::static_type()])
                    .build(),
                Signal::builder("duration-changed")
                    .param_types([u32::static_type(), u32::static_type(), u64::static_type()])
                    .build(),
                Signal::builder("track-cleared")
                    .param_types([u32::static_type(), u32::static_type()])
                    .build(),
                Signal::builder("queue-changed")
                    .param_types([u32::static_type(), u32::static_type(), u32::static_type()])
                    .build(),
                Signal::builder("query-next").build(),
                Signal::builder("query-prev").build(),
            ]
        });

        SIGNALS.as_ref()
    }

    fn constructed(&self) {
        self.parent_constructed();
        self.watch_bus();
        self.pipeline.bus().unwrap().add_signal_watch();
    }

    fn dispose(&self) {
        self.pipeline.bus().unwrap().remove_signal_watch();
    }
}
