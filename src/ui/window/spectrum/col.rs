use gtk::cairo;
use crate::ui::window::spectrum::spectrum::{COL_BRICK_COUNT, Color, GAP};

#[derive(Debug)]
pub struct SpectrumCol {
    spec_min: f64,
    spec_max: f64,
    magnitude: f64,
    colors: Vec<Color>
}

impl SpectrumCol {
    pub fn new(magnitude: f64, spec_min: f64, spec_max: f64, colors: Vec<Color>) -> Self {
        Self {
            magnitude,
            spec_min,
            spec_max,
            colors,
        }
    }

    pub fn draw(&mut self, col_width: f64, col_height: f64, cr: &cairo::Context, x_pos: f64) {
        let brick_h = (col_height - (GAP * COL_BRICK_COUNT as f64 - 1.0)) / COL_BRICK_COUNT as f64;
        if self.spec_max == self.spec_min {
            return;
        }

        let upper = ((self.magnitude - self.spec_min)/(self.spec_max - self.spec_min) * COL_BRICK_COUNT as f64).round().abs() as u64;

        for (i, c) in self.colors.iter().enumerate() {
            if i as u64 >= upper {
                break;
            }

            let y_pos = col_height - (i as f64) * (brick_h + GAP) - brick_h;
            cr.rectangle(x_pos, y_pos, col_width, brick_h);
            cr.set_source_rgba(self.colors[i][0] , self.colors[i][1], self.colors[i][2], self.colors[i][3]);
            cr.fill();

        }
    }
}