use gtk::glib;
use gtk::prelude::{Cast, DrawingAreaExtManual};
use gtk::subclass::prelude::*;
use std::sync::Mutex;

#[derive(Debug)]
pub struct BeatSpectrumImp {
    pub specs: Mutex<Vec<f32>>,
}

impl Default for BeatSpectrumImp {
    fn default() -> Self {
        Self {
            specs: Mutex::new(vec![]),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for BeatSpectrumImp {
    const NAME: &'static str = "BeatSpectrum";
    type Type = super::BeatSpectrum;
    type ParentType = gtk::DrawingArea;
}

impl ObjectImpl for BeatSpectrumImp {
    fn constructed(&self) {
        self.parent_constructed();
        self.obj().set_draw_func(|area, cr, w, h| {
            let area = area.downcast_ref::<super::BeatSpectrum>().unwrap();
            BeatSpectrumImp::draw(area.imp(), cr, w, h);
        });
    }
}

impl WidgetImpl for BeatSpectrumImp {}

impl DrawingAreaImpl for BeatSpectrumImp {}
