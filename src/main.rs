use std::{fs, f64::consts::PI};

use color::{Canvas, Color};
use lighting::{Material, PointLight, lighting};
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
mod lighting;

#[cfg(test)]
#[macro_use]
extern crate approx;

fn main() {
    let mut canvas = Canvas::new(1000, 1000);
    
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;

    let pixel_size = wall_size / (canvas.width as f64);
    let half = wall_size / 2.0;

    let material = Material {
        color: Color::new(1.0, 0.2, 1.0),
        ambient: Material::DEFAULT.ambient,
        diffuse: Material::DEFAULT.diffuse,
        specular: Material::DEFAULT.specular,
        shininess: Material::DEFAULT.shininess,
    };
    let light = PointLight { 
        position: Tuple::point(-10.0, 10.0, -10.0), 
        intensity: Color::new(1.0, 1.0, 1.0) 
    };
    let s = Sphere::new(1).material(&material);
    
    for y in 0..canvas.height - 1 {
        let world_y = half - pixel_size * (y as f64);
        for x in 0..canvas.width - 1  {
            let world_x = -half + pixel_size * (x as f64);
            let position = Tuple::point(world_x, world_y, wall_z);

            let ray = Ray::new(ray_origin, position.subtract(&ray_origin).normalize());
            let xs = s.intersect(&ray);

            match xs.hit() {
                Some(hit) => {
                    let point = ray.position(hit.t);
                    let normal = s.normal_at(&point);
                    let eye = ray.direction.negate();
                    let color = lighting(&s.material, &light, &position, &eye, &normal);
                    canvas.write_pixel(x, y, &color);
                },
                None => ()
            }
        }
    }
    
    let ppm = canvas.generate_ppm();
    fs::write("./examples/chapter-6.ppm", ppm).expect("Unable to output file");
}
