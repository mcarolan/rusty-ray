use std::{f64::consts::PI, fs};

use color::Color;
use lighting::{Material, PointLight};
use ray::Sphere;
use transform::*;
use tuple::Tuple;
use world::{Camera, World, WorldObject};

mod color;
mod lighting;
mod matrix2;
mod matrix3;
mod matrix4;
mod ray;
mod transform;
mod tuple;
mod world;

#[cfg(test)]
#[macro_use]
extern crate approx;

fn main() {
    let room_material = Material::DEFAULT
        .color(&Color::new(1.0, 0.9, 0.9))
        .specular(0.0);

    let floor = Sphere::DEFAULT
        .transform(&scaling(10.0, 0.01, 10.0))
        .material(&room_material);

    let left_wall = Sphere::DEFAULT
        .transform(
            &translation(0.0, 0.0, 5.0)
                .mul_matrix(&rotation_y(-PI / 4.0))
                .mul_matrix(&rotation_x(PI / 2.0))
                .mul_matrix(&scaling(10.0, 0.01, 10.0)),
        )
        .material(&room_material);

    let right_wall = Sphere::DEFAULT
        .transform(
            &translation(0.0, 0.0, 5.0)
                .mul_matrix(&rotation_y(PI / 4.0))
                .mul_matrix(&rotation_x(PI / 2.0))
                .mul_matrix(&scaling(10.0, 0.01, 10.0)),
        )
        .material(&room_material);

    let sphere_large = Sphere::DEFAULT
        .transform(&translation(-0.5, 1.0, 0.5))
        .material(
            &Material::DEFAULT
                .color(&Color::new(0.1, 1.0, 0.5))
                .diffuse(0.7)
                .specular(0.3),
        );

    let sphere_smaller = Sphere::DEFAULT
        .transform(&translation(1.5, 0.5, -0.5).mul_matrix(&scaling(0.5, 0.5, 0.5)))
        .material(
            &Material::DEFAULT
                .color(&Color::new(0.5, 1.0, 0.1))
                .diffuse(0.7)
                .specular(0.3),
        );

    let sphere_smallest = Sphere::DEFAULT
        .transform(&translation(-1.5, 0.33, -0.75).mul_matrix(&scaling(0.33, 0.33, 0.33)))
        .material(
            &Material::DEFAULT
                .color(&Color::new(1.0, 0.8, 0.1))
                .diffuse(0.7)
                .specular(0.3),
        );

    let light = PointLight {
        position: Tuple::point(-10.0, 10.0, -10.0),
        intensity: Color::WHITE,
    };

    let transform = view_transform(
        &Tuple::point(0.0, 1.5, -5.0),
        &Tuple::point(0.0, 1.0, 0.0),
        &Tuple::vector(0.0, 1.0, 0.0),
    );

    let camera = Camera::new(1024, 768, PI / 3.0, transform);

    let world_objects: Vec<&dyn WorldObject> = vec![
        &floor,
        &left_wall,
        &right_wall,
        &sphere_large,
        &sphere_smaller,
        &sphere_smallest,
    ];
    let world = World::new(light, world_objects);

    let canvas = camera.render(&world);

    let ppm = canvas.generate_ppm();
    fs::write("./examples/chapter-7.ppm", ppm).expect("Unable to output file");
}
