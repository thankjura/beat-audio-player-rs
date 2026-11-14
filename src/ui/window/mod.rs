mod files;
mod footer;
mod header;
mod imp;
mod notebook;
mod progress;
mod spectrum;

use crate::ui::window::notebook::Tab;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
pub use notebook::BeatNotebook;
use std::rc::Rc;

glib::wrapper! {
    pub struct BeatWindow(ObjectSubclass<imp::BeatWindowImp>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gtk::Accessible, gio::ActionMap, gio::ActionGroup, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl BeatWindow {
    pub fn new<P: gtk::prelude::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    pub fn selected_tab(&self) -> Rc<Tab> {
        self.imp().notebook.imp().selected_tab()
    }
}
