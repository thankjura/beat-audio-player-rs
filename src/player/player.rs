use gstreamer::{Element, Pipeline, State};
use gstreamer::prelude::{ElementExtManual, GstBinExtManual};
use gstreamer_player::gst;
use gst::prelude::*;


pub struct BeatPlayer {
    pub pipeline: Pipeline,
    file_src: Element,
    volume: Element,
    decode_bin: Element,
    sink: Element,
    spectrum: Element,
    //pub bus: Bus,
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


        BeatPlayer {
            pipeline,
            file_src,
            volume,
            decode_bin,
            sink,
            spectrum,
        }
    }

    pub fn set_uri(&self, uri: &str) {
        self.stop();
        self.file_src.set_property_from_str("location", uri);
    }

    pub fn stop(&self) {
        if let Err(_) = self.pipeline.set_state(State::Null) {
            println!("Can't stop playing");
        }
    }

    pub fn play(&self) {
        if let Ok(_) = self.file_src.property_value("location").get::<&str>() {
            if let Err(_) = self.pipeline.set_state(State::Playing) {
                self.pause();
            }
        }
    }

    pub fn toggle_play(&self) {
        if let (Ok(_val), State::Playing, _) = self.pipeline.state(None) {
            self.pause();
        } else {
            self.play();
        }
    }

    pub fn pause(&self) {
        if let Err(_) = self.pipeline.set_state(State::Paused) {
            println!("Can't pause");
        }
    }

    pub fn set_volume(&self, value: f64) {
        self.volume.set_property("volume", value);
    }

    pub fn destroy(&self) {
        self.pipeline.set_state(State::Null).unwrap();
        self.pipeline.bus().unwrap().remove_signal_watch();
    }

    pub fn set_position_percent(&self, progress: f64) {
        if let Some(duration) = self.get_duration() {
            let seek_value = ((duration as f64 / 100.0) * progress) as u64;
            if let Err(_) = self.pipeline.seek_simple(gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT,  seek_value * gst::ClockTime::SECOND) {
                println!("Can't seek");
            }
        }
    }

    pub fn get_position_progress(&self) -> Option<(u64, f64)> {
        if let Some(position) = self.pipeline.query_position::<gst::ClockTime>() {
            if let Some(duration) = self.get_duration() {
                let progress = (position.seconds() as f64 / duration as f64) * 100.0;
                println!("player position: {}, progress: {}, duration: {}", position.seconds(), progress, duration);
                return Some((position.seconds(), progress));
            }
        }
        None
    }

    pub fn get_duration(&self) -> Option<u64> {
        if let Some(duration) = self.pipeline.query_duration::<gst::ClockTime>() {
            return Some(duration.seconds());
        }
        None
    }
}