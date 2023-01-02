mod connector;
mod imp;

use crate::BeatWindow;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use std::rc::Rc;

glib::wrapper! {
    pub struct BeatApp(ObjectSubclass<imp::BeatAppImp>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap, gio::ApplicationCommandLine;
}

impl BeatApp {
    pub fn new(app_id: &str) -> Self {
        glib::Object::new(&[
            ("application-id", &app_id),
            ("flags", &gio::ApplicationFlags::HANDLES_COMMAND_LINE),
        ])
    }

    pub fn has_window(&self) -> bool {
        let obj = self.imp();
        if obj.window.borrow_mut().is_some() {
            return true;
        };

        false
    }

    pub fn get_window(&self) -> Option<Rc<BeatWindow>> {
        let obj = self.imp();
        if let Some(win) = obj.window.borrow_mut().as_ref() {
            return Some(win.clone());
        }

        None
    }
}
