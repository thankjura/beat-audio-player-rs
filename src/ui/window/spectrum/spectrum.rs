use std::mem;
use gtk::cairo;
use gtk::prelude::WidgetExt;
use gtk::subclass::prelude::ObjectSubclassExt;
use crate::ui::window::spectrum::imp::BeatSpectrumImp;

impl BeatSpectrumImp {
    pub fn redraw(&self, specs: Vec<f32>) {
        let mut guard = self.specs.lock().unwrap();
        mem::replace(&mut *guard, specs);
        self.obj().queue_draw();
    }

    pub fn draw(&self, cr: &cairo::Context, w: i32, h: i32) {
        let guard = self.specs.lock().unwrap();

        if !guard.is_empty() {

            let rgba = (
                guard[0].abs().clamp(0.0, 255.0) as f64,
                guard[2].abs().clamp(0.0, 255.0) as f64,
                guard[4].abs().clamp(0.0, 255.0) as f64, 0.5
            );

            println!("{:#?}", guard);
            println!("{:#?}", rgba);

            cr.set_source_rgba(
                (guard[0].abs() / 100.0) as f64,
                (guard[2].abs() / 100.0) as f64,
                (guard[4].abs() / 100.0) as f64, 1.0);
            cr.paint();

            // cr.set_source_rgba(
            //     255.0,
            //     0.0,
            //     0.0,
            //     1.0);
            // cr.paint();
        }
        println!("draw func");
    }
}