mod notebook;
mod header;
mod widget;
mod tab;
mod files;

use gtk::{gio, glib};

glib::wrapper! {
    pub struct BeatWindow(ObjectSubclass<widget::BeatWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, @implements gio::ActionMap, gio::ActionGroup;
}

impl BeatWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::new(&[("application", app)])
    }
}