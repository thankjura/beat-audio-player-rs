use std::iter::zip;
use std::mem;
use gtk::cairo;
use gtk::prelude::WidgetExt;
use gtk::subclass::prelude::ObjectSubclassExt;
use crate::ui::window::spectrum::col::SpectrumCol;
use crate::ui::window::spectrum::imp::BeatSpectrumImp;

pub type Color = [f64; 4];

pub const COLOR_LOWER: Color = [0.0, 0.8, 0.0, 1.0];
pub const COLOR_UPPER: Color = [1.0, 0.0, 0.0, 1.0];
//pub const COLOR_EXTRM: Color = [1.0, 1.0, 0.0, 1.0];
pub const COL_BRICK_COUNT: u32 = 8;
pub const GAP: f64 = 1.0;

pub fn interpolate_colors() -> Vec<Color> {
    let mut out = vec![COLOR_LOWER];
    if COL_BRICK_COUNT < 2 {
        return out;
    }

    if COL_BRICK_COUNT > 2 {
        let mut step_details = vec![];
        for (left, right) in zip(COLOR_LOWER.into_iter(), COLOR_UPPER.into_iter()) {
            let delta = right - left / ((COL_BRICK_COUNT - 1) as f64);
            step_details.push(delta);
        }

        for _s in 0..(COL_BRICK_COUNT - 2) {
            let mut c = vec![];
            for (index, value) in out.last().unwrap().into_iter().enumerate() {
                c.push(value + step_details[index]);
            }
            out.push([c[0], c[1], c[2], c[3]]);
        }
    }

    out.push(COLOR_UPPER);
    out
}

impl BeatSpectrumImp {
    pub fn redraw(&self, specs: Vec<f32>) {
        let mut guard = self.specs.lock().unwrap();
        let value = mem::replace(&mut *guard, specs);
        drop(value);
        self.obj().queue_draw();
    }

    pub fn clear(&self) {
        let mut guard = self.specs.lock().unwrap();
        guard.clear();
        self.obj().queue_draw();
    }

    pub fn draw(&self, cr: &cairo::Context, w: i32, h: i32) {
        let guard = self.specs.lock().unwrap();
        if guard.is_empty() {
            cr.push_group();
            cr.pop_group_to_source().unwrap();
            cr.paint().unwrap();
            return;
        }

        let spec_max = *guard.iter().max_by(|a, b| a.total_cmp(b)).unwrap() as f64;
        let spec_min = *guard.iter().min_by(|a, b| a.total_cmp(b)).unwrap() as f64;

        let mut cols = vec![];

        let colors = self.colors.lock().unwrap().clone();

        for spec in guard.iter() {
            cols.push(SpectrumCol::new(*spec as f64, spec_min, spec_max, colors.clone()));
        }

        // let w = self.obj().allocated_width() as f64;
        // let h = self.obj().allocated_height() as f64;

        let w = w as f64;
        let h = h as f64;
        cr.push_group();

        let cols_count = cols.len() as f64;
        let col_width = (w - (GAP * (cols_count - 1.0))) / cols_count;
        let mut x_pos = w - GAP - col_width;

        for (i, col) in cols.iter_mut().enumerate() {
            if i != 0usize {
                x_pos -= GAP + col_width;
            }

            col.draw(col_width, h, cr, x_pos);
        }

        cr.pop_group_to_source().unwrap();
        cr.paint().unwrap();
    }
}