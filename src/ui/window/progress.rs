use crate::ui::window::imp::BeatWindowImp;

impl BeatWindowImp {
    pub fn update_progress(&self, duration: u64, position: u64) {
        println!("{}, {}", duration, position);
    }
}