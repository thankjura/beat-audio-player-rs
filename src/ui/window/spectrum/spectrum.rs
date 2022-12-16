use std::iter::zip;
use std::mem;
use gtk::cairo;
use gtk::prelude::WidgetExt;
use gtk::subclass::prelude::ObjectSubclassExt;
use crate::ui::window::spectrum::imp::BeatSpectrumImp;

pub type Color = [f64; 4];

pub const COLOR_LOWER: Color = [0.0, 0.8, 0.0, 1.0];
pub const COLOR_UPPER: Color = [1.0, 0.0, 0.0, 1.0];
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

pub fn draw_column(magnitude: f64, spec_min: f64, spec_max: f64, colors: &Vec<Color>, col_width: f64, col_height: f64, cr: &cairo::Context, x_pos: f64) {

    if spec_max == spec_min {
        return;
    }

    let brick_h = (col_height - (GAP * COL_BRICK_COUNT as f64 - 1.0)) / COL_BRICK_COUNT as f64;


    let upper = ((magnitude - spec_min)/(spec_max - spec_min) * COL_BRICK_COUNT as f64).round().abs() as u64;

    for (i, c) in colors.iter().enumerate() {
        if i as u64 >= upper {
            break;
        }

        let y_pos = col_height - (i as f64) * (brick_h + GAP) - brick_h;
        cr.rectangle(x_pos, y_pos, col_width, brick_h);
        cr.set_source_rgba(c[0] , c[1], c[2], c[3]);
        cr.fill().unwrap();

    }
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

        let colors = self.colors.lock().unwrap().clone();

        let w = w as f64;
        let h = h as f64;
        cr.push_group();

        let cols_count = guard.len() as f64;

        let col_width = (w - (GAP * (cols_count as f64 - 1.0))) / cols_count;
        let mut x_pos = w - GAP - col_width;

        for (i, spec) in guard.iter().enumerate() {
            if i != 0usize {
                x_pos -= GAP + col_width;
            }

            draw_column(*spec as f64, spec_min, spec_max, &colors, col_width, h, cr, x_pos);
        }

        cr.pop_group_to_source().unwrap();
        cr.paint().unwrap();
    }
}