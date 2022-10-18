use std::{f64::consts::PI, fs};

use color::Color;
use lighting::{Material, PointLight};
use plane::Plane;
use ray::Sphere;
use transform::*;
use tuple::Tuple;
use world::{Camera, World, WorldObject};

mod color;
mod lighting;
mod matrix2;
mod matrix3;
mod matrix4;
mod plane;
mod ray;
mod transform;
mod tuple;
mod world;

#[cfg(test)]
#[macro_use]
extern crate approx;

fn main() {
    let wall_material = Material::DEFAULT
        .color(&Color::new(1.0, 0.9, 0.9))
        .ambient(0.05)
        .diffuse(0.6)
        .specular(0.0);

    let floor = Plane::DEFAULT.material(&wall_material.diffuse(1.2).ambient(0.1));
    let wall1 = Plane::DEFAULT.material(&wall_material).transform(
        &translation(0.0, 0.0, 10.0).mul_matrix(&rotation_x(std::f64::consts::PI / 2.0)),
    );
    let wall2 = Plane::DEFAULT.material(&wall_material).transform(
        &&translation(-8.0, 0.0, 10.0)
            .mul_matrix(&rotation_y(90.0 / 180.0 * -std::f64::consts::PI))
            .mul_matrix(&rotation_x(std::f64::consts::PI / 2.0)),
    );

    let sphere_material = Material::DEFAULT
        .color(&Color::new(0.1, 1.0, 0.5))
        .diffuse(0.7)
        .specular(0.3);
    let sphere = Sphere::DEFAULT.material(&sphere_material).transform(
        &translation(1.5, 0.0, -1.0)
            .mul_matrix(&rotation_z(std::f64::consts::PI / 2.0))
            .mul_matrix(&scaling(0.4, 2.2, 1.8)),
    );

    let sphere2 = Sphere::DEFAULT
        .material(
            &sphere_material
                .diffuse(0.5)
                .specular(0.3)
                .color(&Color::new(0.5, 0.1, 1.0)),
        )
        .transform(
            &translation(1.5, 0.4, -1.0)
                .mul_matrix(&rotation_z(std::f64::consts::PI / 2.0))
                .mul_matrix(&rotation_x(std::f64::consts::PI / 4.0))
                .mul_matrix(&scaling(1.6, 0.2, 1.6)),
        );

    let sphere3 = Sphere::DEFAULT
        .material(
            &sphere_material
                .diffuse(0.5)
                .specular(0.3)
                .color(&Color::new(0.1, 0.5, 0.1)),
        )
        .transform(
            &translation(1.5, 0.4, -1.0)
                .mul_matrix(&rotation_z(std::f64::consts::PI / 2.0))
                .mul_matrix(&rotation_x(std::f64::consts::PI / 3.0))
                .mul_matrix(&scaling(5.0, 0.2, 1.6)),
        );

    let light = PointLight {
        position: Tuple::point(5.0, 1.0, -10.0),
        intensity: Color::WHITE,
    };

    let transform = view_transform(
        &Tuple::point(0.0, 2.0, -5.0),
        &Tuple::point(0.0, 2.0, 0.0),
        &Tuple::vector(0.0, 1.0, 0.0),
    );

    let camera = Camera::new(800, 600, PI / 1.5, transform);

    let world_objects: Vec<&dyn WorldObject> =
        vec![&sphere, &floor, &wall1, &wall2, &sphere2, &sphere3];
    let world = World::new(light, world_objects);

    let canvas = camera.render(&world);

    let ppm = canvas.generate_ppm();
    fs::write("./examples/chapter-9.ppm", ppm).expect("Unable to output file");
}
