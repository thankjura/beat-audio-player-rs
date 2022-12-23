use crate::ui::window::imp::BeatWindowImp;
use crate::utils::format::{time_str, NON_TIME_STRING};
use gtk::prelude::RangeExt;

impl BeatWindowImp {
    pub fn update_progress(&self, position: u64, progress: f64) {
        if progress >= 0.0 {
            let progress = progress.clamp(0.0, 100.0);
            self.progress.get().set_value(progress);
            self.current_position_label
                .get()
                .set_label(&time_str(position));
        } else {
            self.clear_duration();
        }
    }
    pub fn update_duration(&self, tab_idx: u32, track_idx: u32, duration: u64) {
        self.duration_label.get().set_label(&time_str(duration));
        self.notebook
            .set_track_duration(tab_idx, track_idx, duration);
    }

    pub fn clear_duration(&self) {
        self.current_position_label.get().set_label(NON_TIME_STRING);
        self.duration_label.get().set_label(NON_TIME_STRING);
        self.progress.get().set_value(0.0);
    }
}
