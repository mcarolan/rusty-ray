use std::{fs, f64::consts::PI};

use color::{Canvas, Color};
use tuple::Tuple;

mod tuple;
mod color;
mod matrix4;
mod matrix2;
mod matrix3;
mod transform;

#[cfg(test)]
#[macro_use]
extern crate approx;

fn main() {
    let mut canvas = Canvas::new(120, 120);
    let red = Color::new(1.0, 0.0, 0.0);
    
    let mut points = Vec::new();

    let twelve = Tuple::point(0.0, 0.0, 1.0);

    for i in 0..12 {
        let rads = (i as f64) * (PI / 6.0);
        points.push(transform::rotation_y(rads).mul_tuple(&twelve));
    }

    let scaled_translated: Vec<Tuple> = points.iter().map(|p| {
        transform::translation(60.0, 0.0, 60.0).mul_matrix(&transform::scaling(50.0, 0.0, 50.0)).mul_tuple(&p)
    }).collect();

    for p in scaled_translated.iter() {
        canvas.write_pixel(p.x as i64, p.z as i64, &red);
    }
    
    let ppm = canvas.generate_ppm();
    fs::write("./examples/chapter-4.ppm", ppm).expect("Unable to output file");
}
