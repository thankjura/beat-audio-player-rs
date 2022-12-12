mod imp;
mod window_actions;

use std::path::Path;
use std::rc::Rc;
use gtk::{gio, glib};
use gtk::subclass::prelude::*;
use crate::BeatWindow;
use crate::ui::Track;


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

    pub fn open_path(&self, paths: &Vec<&str>, append: bool) {
        let mut tab = self.window().unwrap().selected_tab();
        if !append && tab.has_tracks() {
            tab = self.window().unwrap().imp().notebook.imp().add_tab("new");
        }
        for path in paths {
            let path = Path::new(path);
            if path.is_file() {
                tab.add_track(Track::new(path));
            }
        }
    }
}