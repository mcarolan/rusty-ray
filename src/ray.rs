use crate::{
    matrix4::Matrix4,
    tuple::Tuple,
    object::Object
};
use approx::abs_diff_eq;

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn position(&self, t: f64) -> Tuple {
        self.origin.add(&self.direction.scalar_mul(t))
    }

    pub fn transform(&self, m: &Matrix4) -> Ray {
        Ray {
            origin: m.mul_tuple(&self.origin),
            direction: m.mul_tuple(&self.direction),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Intersection<'a> {
    pub t: f64,
    pub obj: &'a Object,
}

impl PartialEq for Intersection<'_> {
    fn eq(&self, other: &Self) -> bool {
        abs_diff_eq!(self.t, other.t)
    }
}

impl PartialOrd for Intersection<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.t.partial_cmp(&other.t)
    }
}

pub struct Intersections<'a> {
    pub values: Vec<Intersection<'a>>,
}

impl Intersections<'_> {
    pub fn hit(&self) -> Option<&Intersection> {
        self.values
            .iter()
            .filter(|v| v.t >= 0.0)
            .fold(None, |a_opt, b| match a_opt {
                None => Some(b),
                Some(a) if a.t < b.t => Some(a),
                _ => Some(b),
            })
    }

    pub fn intersect<'a>(object: &'a Object, ray: &Ray) -> Intersections<'a> {
        let object_ray = ray.transform(&object.transform.inverse());
        object.object_intersect(&object_ray)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lighting::Material,
        transform,
        tuple::Tuple,
        object::{Object, ObjectType},
    };

    use super::{Intersection, Intersections, Ray};

    #[test]
    fn ray_point_distance() {
        let ray = Ray::new(Tuple::point(2.0, 3.0, 4.0), Tuple::vector(1.0, 0.0, 0.0));

        assert_abs_diff_eq!(ray.position(0.0), Tuple::point(2.0, 3.0, 4.0));
        assert_abs_diff_eq!(ray.position(1.0), Tuple::point(3.0, 3.0, 4.0));
        assert_abs_diff_eq!(ray.position(-1.0), Tuple::point(1.0, 3.0, 4.0));
        assert_abs_diff_eq!(ray.position(2.5), Tuple::point(4.5, 3.0, 4.0));
    }

    #[test]
    fn sphere_intersect_two_points() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = Intersections::intersect(&Object::SPHERE, &ray);
        assert_eq!(xs.values.len(), 2);
        assert_abs_diff_eq!(xs.values[0].t, 4.0);
        assert_abs_diff_eq!(xs.values[1].t, 6.0);
    }

    #[test]
    fn sphere_intersect_one_point() {
        let ray = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = Intersections::intersect(&Object::SPHERE, &ray);
        assert_eq!(xs.values.len(), 2);
        assert_abs_diff_eq!(xs.values[0].t, 5.0);
        assert_abs_diff_eq!(xs.values[1].t, 5.0);
    }

    #[test]
    fn sphere_intersect_miss() {
        let ray = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = Intersections::intersect(&Object::SPHERE, &ray);
        assert_eq!(xs.values.len(), 0);
    }

    #[test]
    fn sphere_intersect_inside() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = Intersections::intersect(&Object::SPHERE, &ray);
        assert_eq!(xs.values.len(), 2);
        assert_abs_diff_eq!(xs.values[0].t, -1.0);
        assert_abs_diff_eq!(xs.values[1].t, 1.0);
    }

    #[test]
    fn sphere_intersect_behind() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = Intersections::intersect(&Object::SPHERE, &ray);
        assert_eq!(xs.values.len(), 2);
        assert_abs_diff_eq!(xs.values[0].t, -6.0);
        assert_abs_diff_eq!(xs.values[1].t, -4.0);
    }

    #[test]
    fn intersections_all_positive() {
        let i1 = Intersection {
            obj: &Object::SPHERE,
            t: 1.0,
        };
        let i2 = Intersection {
            obj: &Object::SPHERE,
            t: 2.0,
        };
        let xs = Intersections {
            values: vec![i1, i2],
        };
        assert_eq!(xs.hit().map(|i| { i.t }), Some(i1.t));
    }

    #[test]
    fn intersections_negative() {
        let i1 = Intersection {
            obj: &Object::SPHERE,
            t: -1.0,
        };
        let i2 = Intersection {
            obj: &Object::SPHERE,
            t: 1.0,
        };
        let xs = Intersections {
            values: vec![i2, i1],
        };
        assert_eq!(xs.hit().map(|i| { i.t }), Some(i2.t));
    }

    #[test]
    fn intersections_all_negative() {
        let i1 = Intersection {
            obj: &Object::SPHERE,
            t: -2.0,
        };
        let i2 = Intersection {
            obj: &Object::SPHERE,
            t: -1.0,
        };
        let xs = Intersections {
            values: vec![i2, i1],
        };
        assert_eq!(xs.hit().map(|i| { i.t }), None);
    }

    #[test]
    fn intersections_smallest_non_negative() {
        let i1 = Intersection {
            obj: &Object::SPHERE,
            t: 5.0,
        };
        let i2 = Intersection {
            obj: &Object::SPHERE,
            t: 7.0,
        };
        let i3 = Intersection {
            obj: &Object::SPHERE,
            t: -3.0,
        };
        let i4 = Intersection {
            obj: &Object::SPHERE,
            t: 2.0,
        };
        let xs = Intersections {
            values: vec![i1, i2, i3, i4],
        };
        assert_eq!(xs.hit().map(|i| { i.t }), Some(i4.t));
    }

    #[test]
    fn ray_transform() {
        let ray = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
        let m = transform::translation(3.0, 4.0, 5.0);
        let ray2 = ray.transform(&m);
        assert_abs_diff_eq!(ray2.origin, Tuple::point(4.0, 6.0, 8.0));
        assert_abs_diff_eq!(ray2.direction, Tuple::vector(0.0, 1.0, 0.0));

        let m2 = transform::scaling(2.0, 3.0, 4.0);
        let ray3 = ray.transform(&m2);
        assert_abs_diff_eq!(ray3.origin, Tuple::point(2.0, 6.0, 12.0));
        assert_abs_diff_eq!(ray3.direction, Tuple::vector(0.0, 3.0, 0.0));
    }

    #[test]
    fn ray_sphere_scaled() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Object {
            object_type: ObjectType::Sphere,
            transform: transform::scaling(2.0, 2.0, 2.0),
            material: Material::DEFAULT,
        };
        let xs = Intersections::intersect(&s, &r);
        assert_eq!(xs.values.len(), 2);
        assert_abs_diff_eq!(xs.values[0].t, 3.0);
        assert_abs_diff_eq!(xs.values[1].t, 7.0);
    }

    #[test]
    fn ray_sphere_translated() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Object {
            object_type: ObjectType::Sphere,
            transform: transform::translation(5.0, 0.0, 0.0),
            material: Material::DEFAULT,
        };
        let xs = Intersections::intersect(&s, &r);
        assert_eq!(xs.values.len(), 0);
    }

}
