mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct BeatFooter(ObjectSubclass<imp::BeatFooter>)
        @extends gtk::Widget;
}
