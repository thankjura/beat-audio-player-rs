use std::time::Duration;
use gstreamer::{Bus, Element, Pipeline, State, StateChangeError, StateChangeSuccess};
use gstreamer::prelude::{ElementExtManual, GstBinExtManual};
use gstreamer_player::gst;
use gst::prelude::*;
use gtk::glib;


pub struct BeatPlayer {
    pipeline: Pipeline,
    file_src: Element,
    volume: Element,
    decode_bin: Element,
    sink: Element,
    spectrum: Element,
    bus: Bus,
}


impl Default for BeatPlayer {
    fn default() -> Self {
        Self::new()
    }
}


impl BeatPlayer {
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

        let bus = pipeline.bus().unwrap();
        bus.add_signal_watch();
        bus.connect("message::stream-start", true, |_| {
            glib::timeout_add(Duration::from_secs(1), || {
                println!("Stream started");
                Continue(true)
            });
            None
        });

        BeatPlayer {
            pipeline,
            file_src,
            volume,
            decode_bin,
            sink,
            spectrum,
            bus,
        }
    }

    pub fn set_uri(&self, uri: &str) {
        self.file_src.set_property_from_str("location", uri);
    }

    pub fn stop(&self) -> Result<StateChangeSuccess, StateChangeError> {
        self.pipeline.set_state(State::Null)
    }

    pub fn play(&self) -> Result<(), ()> {
        match self.file_src.property_value("location").get::<&str>() {
            Ok(_) => {
                match self.pipeline.set_state(State::Playing) {
                    Ok(_) => {
                        Ok(())
                    },
                    Err(_) => {
                        Err(())
                    }
                }
            }
            Err(_) => {
                Err(())
            }
        }
    }

    pub fn pause(&self) -> Result<StateChangeSuccess, StateChangeError> {
        self.pipeline.set_state(State::Paused)
    }

    pub fn set_volume(&self, value: f64) {
        self.volume.set_property("volume", value);
    }

    pub fn destroy(&self) {
        self.pipeline.set_state(State::Null).unwrap();
        self.bus.remove_signal_watch();
    }

    pub fn set_position(&self, progress: f64) {
        if let Some(duration) = self.pipeline.query_duration::<gst::ClockTime>() {
            let seek_value = ((duration.seconds() as f64 / 100.0) * progress) as u64;
            self.pipeline.seek_simple(gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT,  seek_value * gst::ClockTime::SECOND);
        }
    }
}