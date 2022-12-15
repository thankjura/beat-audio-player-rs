use std::iter::zip;

type Color = [f32; 4];

const COLOR_LOWER: Color = [0.0, 0.8, 0.0, 1.0];
const COLOR_UPPER: Color = [0.8, 0.0, 0.0, 1.0];
const COLOR_EXTRM: Color = [0.1, 0.8, 0.0, 1.0];


const COUNT: u32 = 8;
const GAP: u32 = 1;


fn interpolate_colors(start: Color, target: Color, steps: u32) -> Vec<Color> {
    let mut out = vec![];
    if steps < 2 {
        return out;
    }

    if steps > 2 {
        let mut step_details = vec![];
        for (left, right) in zip(start.into_iter(), target.into_iter()) {
            let delta = right - left / ((steps - 1) as f32);
            step_details.push(delta);
        }

        for s in 0..(steps - 2) {
            let mut c = vec![];
            for (index, value) in out.last().unwrap().into_iter().enumerate() {
                c.push(value + step_details[index]);
            }
            out.push([c[0], c[1], c[2], c[3]]);
        }
    }

    out.push(target);

    out
}