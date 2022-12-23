mod files;
mod footer;
mod header;
mod imp;
mod notebook;
mod progress;
mod spectrum;

use crate::ui::window::notebook::Tab;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::{gio, glib};
use std::rc::Rc;

pub use notebook::BeatNotebook;

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
