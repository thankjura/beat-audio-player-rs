use std::path::Path;
use crate::ui::window::widget::BeatWindow;

impl BeatWindow {
    pub fn open_path(&self, path: &str, _keep_tab: bool) {
        let path = Path::new(path);
        if path.is_file() {
            // TODO: check type
            //let playlist = self.
        }
    }
}