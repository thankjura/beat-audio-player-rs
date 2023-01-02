mod draw;
mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct BeatSpectrum(ObjectSubclass<imp::BeatSpectrumImp>)
        @extends gtk::Widget, gtk::DrawingArea;
}
