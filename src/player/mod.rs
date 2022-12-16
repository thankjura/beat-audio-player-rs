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

    fn get_track_queue_position(&self, tab_idx: u32, track_idx: u32) -> Option<usize> {
        let guard = self.imp().queue.lock().unwrap();
        let cur_position = guard.iter().position(|t| {
            t.tab_idx == tab_idx && t.track_idx == track_idx
        });

        cur_position
    }

    pub fn add_to_queue(&self, tab_idx: u32, track_idx: u32, filepath: String) {
        //self.imp().queue.lock().unwrap().push_back(TrackRef { tab_idx, track_idx, filepath });
        if let None = self.get_track_queue_position(tab_idx, track_idx) {
            let mut guard = self.imp().queue.lock().unwrap();
            guard.push_back(TrackRef { tab_idx, track_idx, filepath });
            let position = guard.len() as u32;
            self.emit_by_name::<()>("queue-changed", &[&tab_idx, &track_idx, &position]);
        }
    }

    pub fn rm_from_queue(&self, tab_idx: u32, track_idx: u32) {
        if let Some(position) = self.get_track_queue_position(tab_idx, track_idx) {
            let mut queue = self.imp().queue.lock().unwrap();
            queue.remove(position);

            self.emit_by_name::<()>("queue-changed", &[&tab_idx, &track_idx, &0u32]);

            if queue.len() > position {
                for (i, t) in queue.iter().enumerate() {
                    if i >= position {
                        let position = i as u32 + 1;
                        self.emit_by_name::<()>("queue-changed", &[&t.tab_idx, &t.track_idx, &position]);
                    }
                }
            }
        }
    }

    pub fn rm_tab_from_queue(&self, tab_idx: u32) {
        let mut queue = self.imp().queue.lock().unwrap();
        let mut i = 0;
        let mut flag = false;

        while i < queue.len() {
            let t = &queue[i];
            if t.tab_idx == tab_idx {
                queue.remove(i);
                flag = true;
            } else {
                i += 1;
                if flag {
                    let position = i as u32;
                    self.emit_by_name::<()>("queue-changed", &[&t.tab_idx, &t.track_idx, &position]);
                }
            }
        }
    }

    pub fn seek_position(&self, percent: f64) {
        self.imp().set_position_percent_smooth(percent);
    }

    pub fn set_volume(&self, value: f64) {
        self.imp().__set_volume(value);
    }

    pub fn destroy(&self) {
        self.imp().destroy();
    }

    fn __on_state_changed(&self, state: State) {
        let mut current_tab = -1;
        let mut current_track = -1;
        let mut current_path = "".to_string();
        if let Some(track) = &self.imp().current_track() {
            current_tab = track.tab_idx as i32;
            current_track = track.track_idx as i32;
            current_path = track.filepath.clone();
        }

        self.emit_by_name::<()>("state-changed", &[&current_tab, &current_track, &current_path, &state]);
    }

    fn __on_stream_start(&self) {
        if let Some(duration) = self.imp().__get_duration() {
            self.emit_by_name::<()>("duration-changed", &[&duration]);
        }
    }

    fn __tick(&self) -> glib::Continue {
        if self.imp().seek_timeout.lock().unwrap().is_none() {
            if let Some((position, progress)) = self.imp().__get_position_progress() {
                self.emit_by_name::<()>("progress-changed", &[&position, &progress]);
            }
        }
        glib::Continue(true)
    }

    fn __on_error(&self) {
        self.next();
    }

    fn __on_eos(&self) {
        self.next();
    }
}