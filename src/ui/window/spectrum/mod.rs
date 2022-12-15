mod utils;
mod col;
mod imp;
mod spectrum;

use gtk::glib;

glib::wrapper! {
    pub struct BeatSpectrum(ObjectSubclass<imp::BeatSpectrumImp>)
        @extends gtk::Widget, gtk::DrawingArea;
}
