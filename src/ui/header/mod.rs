mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct BeatHeader(ObjectSubclass<imp::BeatHeader>)
        @extends gtk::Widget, gtk::HeaderBar;
}


// impl Default for BeatHeader {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl BeatHeader {
//     pub fn new() -> Self {
//         glib::Object::new(&[])
//     }
// }