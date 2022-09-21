use std::fs;

use color::{Canvas, Color};

use crate::tuple::Tuple;

mod tuple;
mod color;
#[macro_use]
extern crate approx;


#[derive(Debug)]
struct Projectile {
    position: tuple::Tuple,
    velocity: tuple::Tuple
}

struct Environment {
    gravity: tuple::Tuple,
    wind: tuple::Tuple
}

fn tick(environment: &Environment, projectile: Projectile) -> Projectile {
    let position = projectile.position.add(&projectile.velocity);
    let velocity  = projectile.velocity.add(&environment.gravity).add(&environment.wind);
    Projectile {
        position: position,
        velocity: velocity
    }
}

fn main() {
    let mut p = Projectile {
        position: tuple::Tuple::point(0.0, 1.0, 0.0),
        velocity: tuple::Tuple::vector(1.0, 1.8, 0.0).normalize().scalar_mul(11.25)
    };

    let e = Environment {
        gravity: tuple::Tuple::vector(0.0, -0.1, 0.0),
        wind: tuple::Tuple::vector(-0.01, 0.0, 0.0)
    };

    let mut canvas = Canvas::new(900, 550);
    let red = Color::new(1.0, 0.0, 0.0);
    while p.position.y >= 0.0 {
        let x = p.position.x.round() as i64;
        let y = (canvas.height - 1) - p.position.y.round() as i64;
        canvas.write_pixel(x, y, &red);
        p = tick(&e, p);
    }
    let ppm = canvas.generate_ppm();
    fs::write("./examples/chapter-2.ppm", ppm).expect("Unable to output file");
}
