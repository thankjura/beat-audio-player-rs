mod imp;
mod header;
mod notebook;
mod progress;

use std::rc::Rc;
use gtk::{gio, glib};
use gtk::subclass::prelude::ObjectSubclassIsExt;
use crate::ui::window::notebook::Tab;

pub use notebook::BeatNotebook;
pub use notebook::Track;

glib::wrapper! {
    pub struct BeatWindow(ObjectSubclass<imp::BeatWindowImp>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup, gtk::ConstraintTarget, gtk::Native, gtk::Root;
}

impl BeatWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::new(&[("application", app)])
    }

    pub fn selected_tab(&self) -> Rc<Tab> {
        self.imp().notebook.imp().selected_tab()
    }

    pub fn set_playing_icon(&self, value: bool) {
        if value {
            self.imp().button_play_img.get().set_from_icon_name(Some("media-playback-pause-symbolic"))
        } else {
            self.imp().button_play_img.get().set_from_icon_name(Some("media-playback-start-symbolic"));
        }
    }
}

