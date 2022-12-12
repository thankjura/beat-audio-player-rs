use std::path::Path;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use crate::structs::track::TrackState;

#[derive(Debug, Default)]
pub struct CellTrackStateImp {
    image: gtk::Image
}

const ACTIVE_ICON: &str = "../../../../../resources/icons/active.svg";
const PLAY_ICON: &str = "../../../../../resources/icons/play.svg";
const PAUSE_ICON: &str = "../../../../../resources/icons/pause.svg";

#[glib::object_subclass]
impl ObjectSubclass for CellTrackStateImp {
    const NAME: &'static str = "CellTrackState";
    type Type = super::CellTrackState;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();
    }
}

impl CellTrackStateImp {
    pub fn set_state(&self, _state: &TrackState) {
        self.image.set_from_file(Some(Path::new(ACTIVE_ICON)));
    }
}

impl ObjectImpl for CellTrackStateImp {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        self.image.set_icon_size(gtk::IconSize::Inherit);
        self.image.set_parent(&*obj);
        //self.pixbuf_cell.set_parent(&*obj);
    }

    fn dispose(&self) {
        while let Some(child) = self.obj().first_child() {
            child.unparent();
        }
    }

}
impl WidgetImpl for CellTrackStateImp {}