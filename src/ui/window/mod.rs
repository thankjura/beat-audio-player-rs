mod imp;

use gtk::{gio, glib};

glib::wrapper! {
    pub struct BeatWindow(ObjectSubclass<imp::BeatWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, @implements gio::ActionMap, gio::ActionGroup;
}

impl BeatWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::new(&[("application", app)])
    }

    // pub fn init_label(&self) {
    //     let imp = self.imp();
    //     imp.subtitle
    //         .set_text("This is an example window made using composite templates");
    // }
}