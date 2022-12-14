mod imp;
mod connector;

use std::path::Path;
use gtk::{gio, glib};
use gtk::subclass::prelude::*;
use crate::structs::track::Track;


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

    pub fn has_window(&self) -> bool {
        let obj = self.imp();
        if let Some(_) = obj.window.borrow_mut().as_ref() {
            return true;
        };

        false
    }

    pub fn open_path(&self, paths: &Vec<&str>, append: bool) {
        let mut tab = self.imp().window.borrow().as_ref().unwrap().selected_tab();
        if !append && tab.has_tracks() {
            tab = self.imp().window.borrow().as_ref().unwrap().imp().notebook.imp().add_tab("new");
        }

        for path in paths {
            let path = Path::new(path);
            if path.is_file() {
                tab.add_track(Track::new(path));
            }
        }
    }
}