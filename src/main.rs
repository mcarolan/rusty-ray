use std::{f64::consts::PI, fs};

use color::Color;
use lighting::{Material, PointLight};
use matrix4::Matrix4;
use object::*;
use pattern::Pattern;
use transform::*;
use tuple::Tuple;
use world::{Camera, World};

mod color;
mod lighting;
mod matrix2;
mod matrix3;
mod matrix4;
mod object;
mod pattern;
mod plane;
mod ray;
mod sphere;
mod transform;
mod tuple;
mod world;

#[cfg(test)]
#[macro_use]
extern crate approx;

fn main() {

    let floor_material = Material {
        pattern: Pattern::Checker {
            a: Color::new(0.0, 1.0,0.0),
            b: Color::new(1.0, 1.0, 1.0),
            transform:  Matrix4::IDENTITY
        },
        ambient: 0.04,
        diffuse: 0.6,
        specular: 0.7,
        shininess: 200.0,
    };

    let floor = Object {
        object_type: ObjectType::Plane,
        material: floor_material,
        transform: Matrix4::IDENTITY
    };
    let wall1 = Object {
        object_type: ObjectType::Plane,
        material: Material {
            pattern: Pattern::Constant {
                value: Color::new(1.0, 0.9, 0.9),
            },
            ambient: 0.05,
            diffuse: 0.6,
            specular: 0.0,
            shininess: 200.0,
        },
        transform: translation(0.0, 0.0, 10.0).mul_matrix(&rotation_x(std::f64::consts::PI / 2.0)),
    };

    let wall2 = Object {
        object_type: ObjectType::Plane,
        material: Material {
            pattern: Pattern::Ring {
                a: Color::new(1.0, 0.0,  0.0),
                b: Color::new(0.0, 0.0, 1.0),
                transform: Matrix4::IDENTITY,
            },
            ambient: 0.05,
            diffuse: 0.6,
            specular: 0.0,
            shininess: 200.0,
        },
        transform: translation(-8.0, 0.0, 10.0)
            .mul_matrix(&rotation_y(90.0 / 180.0 * -std::f64::consts::PI))
            .mul_matrix(&rotation_x(std::f64::consts::PI / 2.0)),
    };

    let sphere_material = Material {
        pattern: Pattern::Constant {
            value: Color::new(0.1, 1.0, 0.5),
        },
        ambient: Material::DEFAULT_AMBIENT,
        diffuse: 0.7,
        specular: 0.3,
        shininess: Material::DEFAULT_SHININESS,
    };

    let sphere = Object {
        object_type: ObjectType::Sphere,
        transform: translation(0.0, 1.5, -1.0),
        material: sphere_material,
    };

    let sphere_material2 = Material {
        pattern: Pattern::Gradient {
            a: Color::WHITE,
            b: Color::BLACK,
            transform: rotation_x(std::f64::consts::PI),
        },
        ambient: Material::DEFAULT_AMBIENT,
        diffuse: 0.7,
        specular: 0.3,
        shininess: Material::DEFAULT_SHININESS,
    };

    let sphere2 = Object {
        object_type: ObjectType::Sphere,
        transform: translation(2.0, 1.5, -0.5),
        material: sphere_material2,
    };

    let sphere_material3 = Material {
        pattern: Pattern::Stripe {
            a: Color::WHITE,
            b: Color::BLACK,
            transform: scaling(0.2, 0.2, 0.2),
        },
        ambient: Material::DEFAULT_AMBIENT,
        diffuse: 0.7,
        specular: 0.3,
        shininess: Material::DEFAULT_SHININESS,
    };

    let sphere3 = Object {
        object_type: ObjectType::Sphere,
        transform: translation(-2.0, 1.0, -1.0).mul_matrix(&rotation_y(std::f64::consts::PI / 2.0)),
        material: sphere_material3,
    };

    let light = PointLight {
        position: Tuple::point(5.0, 3.0, -10.0),
        intensity: Color::WHITE,
    };

    let transform = view_transform(
        &Tuple::point(0.0, 2.0, -5.0),
        &Tuple::point(0.0, 2.0, 0.0),
        &Tuple::vector(0.0, 1.0, 0.0),
    );

    let camera = Camera::new(1024, 768, PI / 1.5, transform);

    let world_objects: Vec<Object> = vec![sphere, sphere2, sphere3, floor, wall1, wall2];
    let world = World::new(light, world_objects);

    let canvas = camera.render(&world);

    let ppm = canvas.generate_ppm();
    fs::write("./examples/chapter-10.ppm", ppm).expect("Unable to output file");
}
