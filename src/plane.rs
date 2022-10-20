use crate::{
    ray::{Intersection, Intersections},
    world::World,
    object::Object
};

pub fn plane_object_intersect<'a>(obj: &'a Object, ray: &crate::ray::Ray) -> crate::ray::Intersections<'a> {
    if ray.direction.y.abs() < World::EPSILON {
        Intersections { values: Vec::new() }
    } else {
        let t = -ray.origin.y / ray.direction.y;
        Intersections {
            values: vec![Intersection { t, obj: obj }],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ray::Ray, tuple::Tuple, object::Object
    };

    #[test]
    fn normal_constant_everywhere() {
        assert_abs_diff_eq!(
            Object::PLANE.object_normal(&Tuple::point(0.0, 0.0, 0.0)),
            Tuple::vector(0.0, 1.0, 0.0)
        );
        assert_abs_diff_eq!(
            Object::PLANE.object_normal(&Tuple::point(10.0, 0.0, -10.0)),
            Tuple::vector(0.0, 1.0, 0.0)
        );
        assert_abs_diff_eq!(
            Object::PLANE.object_normal(&Tuple::point(-5.0, 0.0, 150.0)),
            Tuple::vector(0.0, 1.0, 0.0)
        );
    }

    #[test]
    fn plane_no_intersection() {
        let r = Ray::new(Tuple::point(0.0, 10.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        assert!(Object::PLANE.object_intersect(&r).values.is_empty());

        let r2 = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        assert!(Object::PLANE.object_intersect(&r2).values.is_empty());
    }

    #[test]
    fn plane_intersections() {
        let r = Ray::new(Tuple::point(0.0, 1.0, 0.0), Tuple::vector(0.0, -1.0, 0.0));
        let xs = Object::PLANE.object_intersect(&r);
        assert_eq!(xs.values.len(), 1);
        assert_abs_diff_eq!(xs.values.first().unwrap().t, 1.0);

        let r2 = Ray::new(Tuple::point(0.0, -1.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));
        let xs2 = Object::PLANE.object_intersect(&r2);
        assert_eq!(xs2.values.len(), 1);
        assert_abs_diff_eq!(xs2.values.first().unwrap().t, 1.0);
    }
}
