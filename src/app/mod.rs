mod connector;
mod imp;
mod remote;

use crate::BeatWindow;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
pub use remote::{Command, CLI_ACTIONS};
use std::rc::Rc;

glib::wrapper! {
    pub struct BeatApp(ObjectSubclass<imp::BeatAppImp>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap, gio::ApplicationCommandLine;
}

impl BeatApp {
    pub fn new(app_id: &str) -> Self {
        glib::Object::builder()
            .property("application-id", app_id)
            .property("flags", gio::ApplicationFlags::HANDLES_COMMAND_LINE)
            .build()
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

    /// Runs a playback or window command, as if it came from MPRIS or the tray.
    pub fn run_command(&self, command: Command) {
        let remote = self.imp().remote.borrow().clone();
        if let Some(remote) = remote {
            remote.handle(command);
        }
    }
}
