use gtk::prelude::ObjectExt;
use gtk::subclass::prelude::ObjectSubclassExt;
use crate::player::imp::BeatPlayerImp;
use crate::player::TrackRef;

impl BeatPlayerImp {
    pub fn play_ref(&self, tab_idx: u32, track_idx: u32, filepath: String) {
        self.__set_uri(&filepath);
        self.__play();
        self.set_current_track(TrackRef {tab_idx, track_idx, filepath});
    }

    pub fn play(&self) {
        if let Some(_track_ref) = self.current_track() {
            self.__toggle_play();
        } else {
            println!("Implement me: play");
        }
    }

    pub fn next(&self) {
        let mut queue = self.queue.lock().unwrap();
        if let Some(t) = queue.pop_front() {
            self.play_ref(t.tab_idx, t.track_idx, t.filepath);
        } else {
            self.obj().emit_by_name::<()>("query-next", &[]);
        }
    }

    pub fn prev(&self) {
        self.obj().emit_by_name::<()>("query-prev", &[]);
    }
}