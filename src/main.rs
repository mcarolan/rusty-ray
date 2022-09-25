use std::{fs, f64::consts::PI};

use color::{Canvas, Color};
use matrix4::Matrix4;
use ray::{Sphere, Ray};
use tuple::Tuple;
use ray::Intersect;

mod tuple;
mod color;
mod matrix4;
mod matrix2;
mod matrix3;
mod transform;
mod ray;

#[cfg(test)]
#[macro_use]
extern crate approx;

fn main() {
    let mut canvas = Canvas::new(100, 100);
    let red = Color::new(1.0, 0.0, 0.0);
    
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;

    let pixel_size = wall_size / (canvas.width as f64);
    let half = wall_size / 2.0;

    let s = Sphere::new(1, Matrix4::IDENTITY);
    
    for y in 0..canvas.height - 1 {
        let world_y = half - pixel_size * (y as f64);
        for x in 0..canvas.width - 1  {
            let world_x = -half + pixel_size * (x as f64);
            let position = Tuple::point(world_x, world_y, wall_z);

            let ray = Ray::new(ray_origin, position.subtract(&ray_origin).normalize());
            let xs = s.intersect(&ray);

            if xs.hit().is_some() {
                canvas.write_pixel(x, y, &red);
            }
        }
    }
    
    let ppm = canvas.generate_ppm();
    fs::write("./examples/chapter-5.ppm", ppm).expect("Unable to output file");
}
