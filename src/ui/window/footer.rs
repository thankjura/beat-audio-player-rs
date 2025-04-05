use crate::ui::window::imp::BeatWindowImp;
use std::path::PathBuf;

impl BeatWindowImp {
    pub fn set_cover(&self, picture: Option<PathBuf>) {
        if let Some(picture) = picture {
            self.cover.set_from_file(Some(picture));
            //self.cover.;
        } else {
            self.cover
                .set_resource(Some("/ru/slie/beat/icons/album.svg"));
        }
    }
}
