mod imp;

use std::rc::Rc;
use gtk::{gio, glib};
use gtk::subclass::prelude::*;
use crate::BeatWindow;


glib::wrapper! {
    pub struct BeatApp(ObjectSubclass<imp::BeatAppImp>)
        @extends gio::Application, gtk::Application,
        @implements gio::ActionGroup, gio::ActionMap, gio::ApplicationCommandLine;
}


impl BeatApp {
    pub fn new(app_id: &str) -> Self {
        glib::Object::new(&[
            ("application-id", &app_id),
            ("flags", &gio::ApplicationFlags::HANDLES_COMMAND_LINE),
        ])
    }

    pub fn window(&self) -> Option<Rc<BeatWindow>> {
        let obj = self.imp();
        if let Some(win) = obj.window.borrow_mut().as_ref() {
            return Some(win.clone());
        };

        None
    }
}