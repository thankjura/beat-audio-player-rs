use gtk::subclass::prelude::*;
use gtk::glib;

#[derive(Default, Debug)]
pub struct BeatSpectrumImp {}

#[glib::object_subclass]
impl ObjectSubclass for BeatSpectrumImp {
    const NAME: &'static str = "BeatSpectrum";
    type Type = super::BeatSpectrum;
    type ParentType = gtk::DrawingArea;
}

impl ObjectImpl for BeatSpectrumImp {}

impl WidgetImpl for BeatSpectrumImp {}

impl DrawingAreaImpl for BeatSpectrumImp {}