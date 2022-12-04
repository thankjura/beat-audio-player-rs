mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct BeatHeader(ObjectSubclass<imp::BeatHeader>)
        @extends gtk::Widget, gtk::HeaderBar;
}
