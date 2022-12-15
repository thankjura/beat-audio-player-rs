use std::sync::Mutex;
use gtk::subclass::prelude::*;
use gtk::glib;
use gtk::prelude::{Cast, DrawingAreaExtManual};
use crate::ui::window::spectrum::BeatSpectrum;
use crate::ui::window::spectrum::spectrum::{Color, interpolate_colors};

#[derive(Debug)]
pub struct BeatSpectrumImp {
    pub specs: Mutex<Vec<f32>>,
    pub colors: Mutex<Vec<Color>>,
}

impl Default for BeatSpectrumImp {
    fn default() -> Self {
        Self {
            specs: Mutex::new(vec![]),
            colors: Mutex::new(interpolate_colors())
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
            let area = area.downcast_ref::<BeatSpectrum>().unwrap();
            BeatSpectrumImp::draw(area.imp(), cr, w, h);
        });
    }
}

impl WidgetImpl for BeatSpectrumImp {}

impl DrawingAreaImpl for BeatSpectrumImp {}