use gtk::prelude::RangeExt;
use crate::ui::format::{NON_TIME_STRING, UFormat};
use crate::ui::window::imp::BeatWindowImp;

impl BeatWindowImp {
    pub fn update_progress(&self, position: u64, progress: f64) {
        let progress = progress.clamp(0.0, 100.0);
        self.progress.get().set_value(progress);
        self.current_position_label.get().set_label(&UFormat::time_str(position));
    }
    pub fn update_duration(&self, duration: u64) {
        self.duration_label.get().set_label(&UFormat::time_str(duration));
    }

    pub fn clear_duration(&self) {
        self.current_position_label.get().set_label(NON_TIME_STRING);
        self.duration_label.get().set_label(NON_TIME_STRING);
        self.progress.get().set_value(0.0);
    }
}