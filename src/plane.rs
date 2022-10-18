use crate::{
    ray::{Intersection, Intersections},
    tuple::Tuple,
    world::{World, WorldObject}, lighting::Material, matrix4::Matrix4,
};

pub struct Plane {
    material: Material,
    transform: Matrix4
}

impl Plane {
    pub const DEFAULT: Plane = Plane {
        material: Material::DEFAULT,
        transform: Matrix4::IDENTITY
    };

    pub fn material(&self, material: &Material) -> Plane {
        Plane {
            material: *material,
            transform: self.transform
        }
    }

    pub fn transform(&self, matrix: &Matrix4) -> Plane {
        Plane {
            material: self.material,
            transform: *matrix
        }
    }
}

impl WorldObject for Plane {
    fn object_normal(&self, _local_point: &crate::tuple::Tuple) -> crate::tuple::Tuple {
        Tuple::vector(0.0, 1.0, 0.0)
    }

    fn material(&self) -> crate::lighting::Material {
        self.material
    }

    fn transform(&self) -> crate::matrix4::Matrix4 {
        self.transform
    }

    fn object_intersect(&self, ray: &crate::ray::Ray) -> crate::ray::Intersections {
        if ray.direction.y < World::EPSILON {
            Intersections { values: Vec::new() }
        } else {
            let t = -ray.origin.y / ray.direction.y;
            Intersections {
                values: vec![Intersection { t, object: self }],
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ray::Ray, tuple::Tuple, world::WorldObject};

    use super::Plane;

    #[test]
    fn normal_constant_everywhere() {
        let p = Plane::DEFAULT;
        assert_abs_diff_eq!(
            p.object_normal(&Tuple::point(0.0, 0.0, 0.0)),
            Tuple::vector(0.0, 1.0, 0.0)
        );
        assert_abs_diff_eq!(
            p.object_normal(&Tuple::point(10.0, 0.0, -10.0)),
            Tuple::vector(0.0, 1.0, 0.0)
        );
        assert_abs_diff_eq!(
            p.object_normal(&Tuple::point(-5.0, 0.0, 150.0)),
            Tuple::vector(0.0, 1.0, 0.0)
        );
    }

    #[test]
    fn plane_no_intersection() {
        let p = Plane::DEFAULT;
        let r = Ray::new(Tuple::point(0.0, 10.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        assert!(p.object_intersect(&r).values.is_empty());

        let r2 = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        assert!(p.object_intersect(&r2).values.is_empty());
    }

    #[test]
    fn plane_intersections() {
        let p = Plane::DEFAULT;
        let r = Ray::new(Tuple::point(0.0, 1.0, 0.0), Tuple::vector(0.0, -1.0, 0.0));

        let r2 = Ray::new(Tuple::point(0.0, -1.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));
        let xs2 = p.object_intersect(&r2);
        assert_eq!(xs2.values.len(), 1);
        assert_abs_diff_eq!(xs2.values.first().unwrap().t, 1.0);
    }
}
