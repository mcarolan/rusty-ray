use crate::{
    color::{Canvas, Color},
    lighting::{lighting, Material, PointLight},
    matrix4::Matrix4,
    ray::{Intersection, Intersections, Ray, Sphere},
    transform,
    tuple::Tuple,
};

pub trait WorldObject {
    fn normal_at(&self, world_point: &Tuple) -> Tuple;
    fn intersect(&self, ray: &Ray) -> Intersections;
    fn material(&self) -> Material;
}

pub struct World<'a> {
    pub light: PointLight,
    pub objects: Vec<&'a dyn WorldObject>,
}

pub struct PreparedComputations {
    point: Tuple,
    eye: Tuple,
    normal: Tuple,
    #[allow(dead_code)]
    is_inside: bool,
}

impl World<'_> {
    #[allow(dead_code)]
    const DEFAULT_LIGHT: PointLight = PointLight {
        position: Tuple::point(-10.0, 10.0, -10.0),
        intensity: Color::WHITE,
    };

    pub fn new<'a>(light: PointLight, objects: Vec<&'a dyn WorldObject>) -> World<'a> {
        World { light, objects }
    }

    #[allow(dead_code)]
    pub fn default() -> World<'static> {
        static S1: Sphere = Sphere::DEFAULT.material(
            &Material::DEFAULT
                .color(&Color::new(0.8, 1.0, 0.6))
                .diffuse(0.7)
                .specular(0.2),
        );
        static S2: Sphere = Sphere::DEFAULT.transform(&transform::scaling(0.5, 0.5, 0.5));
        World {
            light: World::DEFAULT_LIGHT,
            objects: vec![&S1, &S2],
        }
    }

    #[allow(dead_code)]
    pub fn default_with_light(light: PointLight) -> World<'static> {
        static S1: Sphere = Sphere::DEFAULT.material(
            &Material::DEFAULT
                .color(&Color::new(0.8, 1.0, 0.6))
                .diffuse(0.7)
                .specular(0.2),
        );
        static S2: Sphere = Sphere::DEFAULT.transform(&transform::scaling(0.5, 0.5, 0.5));
        World {
            light,
            objects: vec![&S1, &S2],
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let intersections = self.objects.iter().flat_map(|object| {
            let intersections = object.intersect(ray);
            intersections.values
        });
        let mut values = Vec::from_iter(intersections);
        values.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        Intersections { values: values }
    }

    pub fn shade_hit(&self, material: &Material, comps: &PreparedComputations) -> Color {
        lighting(
            material,
            &self.light,
            &comps.point,
            &comps.eye,
            &comps.normal,
        )
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let intersections = self.intersect(&ray);

        match intersections.hit() {
            Some(i) => {
                let comps = World::prepare_computations(i, ray);
                self.shade_hit(&i.object.material(), &comps)
            }
            None => Color::BLACK,
        }
    }

    fn prepare_computations(intersection: &Intersection, ray: &Ray) -> PreparedComputations {
        let point = ray.position(intersection.t);
        let eye = ray.direction.negate();
        let mut normal = intersection.object.normal_at(&point);
        let mut is_inside = false;

        if normal.dot(&eye) < 0.0 {
            normal = normal.negate();
            is_inside = true;
        }

        PreparedComputations {
            point,
            eye,
            normal,
            is_inside,
        }
    }
}

pub struct Camera {
    hsize: i32,
    vsize: i32,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
    inv_transform: Matrix4,
}

impl Camera {
    pub fn new(hsize: i32, vsize: i32, field_of_view: f64, transform: Matrix4) -> Camera {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = f64::from(hsize) / f64::from(vsize);

        let half_width = if aspect >= 1.0 {
            half_view
        } else {
            half_view * aspect
        };
        let half_height = if aspect >= 1.0 {
            half_view / aspect
        } else {
            half_view
        };

        let pixel_size = (half_width * 2.0) / f64::from(hsize);
        let inv_transform = transform.inverse();
        Camera {
            hsize,
            vsize,
            half_width,
            half_height,
            pixel_size,
            inv_transform,
        }
    }

    fn ray_for_pixel(&self, x: i32, y: i32) -> Ray {
        let x_offset = (f64::from(x) + 0.5) * self.pixel_size;
        let y_offset = (f64::from(y) + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let pixel = self
            .inv_transform
            .mul_tuple(&Tuple::point(world_x, world_y, -1.0));
        let origin = self.inv_transform.mul_tuple(&Tuple::point(0.0, 0.0, 0.0));
        let direction = pixel.subtract(&origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: &World<'_>) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let c = world.color_at(&ray);
                image.write_pixel(x, y, &c);
            }
        }
        image
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{
        color::Color,
        lighting::{Material, PointLight},
        matrix4::Matrix4,
        ray::{Intersection, Ray, Sphere},
        transform::{self, rotation_y, translation, view_transform},
        tuple::Tuple,
    };

    use super::{Camera, World};

    #[test]
    fn world_intersect() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let world = World::default();
        let intersections = world.intersect(&ray);

        assert_eq!(intersections.values.len(), 4);
        assert_abs_diff_eq!(intersections.values[0].t, 4.0);
        assert_abs_diff_eq!(intersections.values[1].t, 4.5);
        assert_abs_diff_eq!(intersections.values[2].t, 5.5);
        assert_abs_diff_eq!(intersections.values[3].t, 6.0);
    }

    #[test]
    fn precompute_intersection() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let comps = World::prepare_computations(
            &Intersection {
                object: &Sphere::DEFAULT,
                t: 4.0,
            },
            &ray,
        );

        assert_abs_diff_eq!(comps.point, Tuple::point(0.0, 0.0, -1.0));
        assert_abs_diff_eq!(comps.eye, Tuple::vector(0.0, 0.0, -1.0));
        assert_abs_diff_eq!(comps.normal, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(comps.is_inside, false);
    }

    #[test]
    fn precompute_intersection_inside() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let comps = World::prepare_computations(
            &Intersection {
                object: &Sphere::DEFAULT,
                t: 1.0,
            },
            &ray,
        );

        assert_abs_diff_eq!(comps.point, Tuple::point(0.0, 0.0, 1.0));
        assert_abs_diff_eq!(comps.eye, Tuple::vector(0.0, 0.0, -1.0));
        assert_abs_diff_eq!(comps.normal, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(comps.is_inside, true);
    }

    #[test]
    fn shade_intersection() {
        let world = World::default();
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = *world.objects.first().unwrap();
        let comps = World::prepare_computations(
            &Intersection {
                object: &Sphere::DEFAULT,
                t: 4.0,
            },
            &ray,
        );
        assert_abs_diff_eq!(
            world.shade_hit(&shape.material(), &comps),
            Color::new(0.38066, 0.47583, 0.2855)
        );
    }

    #[test]
    fn shade_intersection_inside() {
        let world = World::default_with_light(PointLight {
            position: Tuple::point(0.0, 0.25, 0.0),
            intensity: Color::WHITE,
        });
        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = *world.objects.get(1).unwrap();
        let comps = World::prepare_computations(
            &Intersection {
                object: &Sphere::DEFAULT,
                t: 0.5,
            },
            &ray,
        );
        assert_abs_diff_eq!(
            world.shade_hit(&shape.material(), &comps),
            Color::new(0.90498, 0.90498, 0.90498)
        );
    }

    #[test]
    fn world_color_at_no_intersection() {
        let world = World::default();
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));
        assert_abs_diff_eq!(world.color_at(&ray), Color::BLACK);
    }

    #[test]
    fn world_color_hit() {
        let world = World::default();
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        assert_abs_diff_eq!(world.color_at(&ray), Color::new(0.38066, 0.47583, 0.2855))
    }

    #[test]
    fn world_color_intersection_behind_ray() {
        let outer = Sphere::DEFAULT.material(
            &&Material::DEFAULT
                .color(&Color::new(0.8, 1.0, 0.6))
                .diffuse(0.7)
                .specular(0.2)
                .ambient(1.0),
        );

        let inner = Sphere::DEFAULT
            .transform(&transform::scaling(0.5, 0.5, 0.5))
            .material(&Material::DEFAULT.ambient(1.0));

        let world = World::new(World::DEFAULT_LIGHT, vec![&outer, &inner]);
        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.75), Tuple::vector(0.0, 0.0, -1.0));
        assert_abs_diff_eq!(world.color_at(&ray), inner.material.color);
    }

    #[test]
    fn camera_pixel_size() {
        let camera = Camera::new(200, 125, PI / 2.0, Matrix4::IDENTITY);
        assert_abs_diff_eq!(camera.pixel_size, 0.01);
        let camera2 = Camera::new(125, 200, PI / 2.0, Matrix4::IDENTITY);
        assert_abs_diff_eq!(camera2.pixel_size, 0.01);
    }

    #[test]
    fn camera_ray_for_pixel_center() {
        let camera = Camera::new(201, 101, PI / 2.0, Matrix4::IDENTITY);
        let r = camera.ray_for_pixel(100, 50);
        assert_abs_diff_eq!(r.origin, Tuple::point(0.0, 0.0, 0.0));
        assert_abs_diff_eq!(r.direction, Tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn camera_ray_for_pixel_corner() {
        let camera = Camera::new(201, 101, PI / 2.0, Matrix4::IDENTITY);
        let r = camera.ray_for_pixel(0, 0);
        assert_abs_diff_eq!(r.origin, Tuple::point(0.0, 0.0, 0.0));
        assert_abs_diff_eq!(r.direction, Tuple::vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn camera_ray_for_pixel_transformed() {
        let camera = Camera::new(
            201,
            101,
            PI / 2.0,
            rotation_y(PI / 4.0).mul_matrix(&translation(0.0, -2.0, 5.0)),
        );
        let r = camera.ray_for_pixel(100, 50);
        assert_abs_diff_eq!(r.origin, Tuple::point(0.0, 2.0, -5.0));
        assert_abs_diff_eq!(
            r.direction,
            Tuple::vector(f64::sqrt(2.0) / 2.0, 0.0, -f64::sqrt(2.0) / 2.0)
        );
    }

    #[test]
    fn camera_render() {
        let world = World::default();
        let from = Tuple::point(0.0, 0.0, -5.0);
        let to = Tuple::point(0.0, 0.0, 0.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);
        let transform = view_transform(&from, &to, &up);
        let camera = Camera::new(11, 11, PI / 2.0, transform);
        let image = camera.render(&world);

        let expected_color = Color::new(0.38066, 0.47583, 0.2855);
        assert_abs_diff_eq!(*image.pixel_at(5, 5).unwrap(), expected_color);
    }
}
