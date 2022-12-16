mod imp;
mod connector;

use gtk::{gio, glib};
use gtk::subclass::prelude::*;


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
        if let Some(_) = obj.window.borrow_mut().as_ref() {
            return true;
        };

        false
    }
}