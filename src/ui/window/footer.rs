use std::path::PathBuf;
use crate::ui::window::imp::BeatWindowImp;


impl BeatWindowImp {
    pub fn set_cover(&self, picture: Option<PathBuf>) {
        if let Some(picture) = picture {
            self.cover.set_filename(Some(picture));
        } else {
            self.cover.set_resource(Some("/ru/slie/beat/icons/album.svg"));
        }
    }
}