use crate::player::imp::BeatPlayerImp;
use crate::player::TrackRef;
use gtk::prelude::ObjectExt;
use gtk::subclass::prelude::ObjectSubclassExt;

impl BeatPlayerImp {
    pub fn play_ref(&self, tab_idx: u32, track_idx: u32, filepath: String) {
        self.__set_uri(&filepath);
        self.__play();
        self.set_current_track(TrackRef {
            tab_idx,
            track_idx,
            filepath,
        });
    }

    pub fn play(&self) {
        if let Some(_track_ref) = self.current_track() {
            self.__toggle_play();
        } else {
            self.next();
        }
    }

    pub fn next(&self) {
        let mut guard = self.queue.lock().unwrap();
        if let Some(track) = guard.pop_front() {
            self.obj()
                .emit_by_name::<()>("queue-changed", &[&track.tab_idx, &track.track_idx, &0u32]);
            self.play_ref(track.tab_idx, track.track_idx, track.filepath);

            for (index, track) in guard.iter().enumerate() {
                let position = index as u32 + 1;
                self.obj().emit_by_name::<()>(
                    "queue-changed",
                    &[&track.tab_idx, &track.track_idx, &position],
                );
            }
        } else {
            self.obj().emit_by_name::<()>("query-next", &[]);
        }
    }

    pub fn prev(&self) {
        self.obj().emit_by_name::<()>("query-prev", &[]);
    }
}
