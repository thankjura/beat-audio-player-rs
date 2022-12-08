mod imp;
mod header;
mod actions;
mod notebook;

use std::rc::Rc;
use gtk::{gio, glib};
use gtk::subclass::prelude::ObjectSubclassIsExt;
use crate::ui::window::notebook::Tab;

pub use notebook::BeatNotebook;
pub use notebook::TrackRef;
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
}

