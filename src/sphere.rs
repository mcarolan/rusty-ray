use crate::{
    object::Object,
    ray::{Intersection, Intersections, Ray},
    tuple::Tuple,
};

pub fn sphere_object_intersect<'a>(sphere: &'a Object, ray: &Ray) -> Intersections<'a> {
    let object_to_ray = ray.origin.subtract(&Tuple::point(0.0, 0.0, 0.0));
    let a = ray.direction.dot(&ray.direction);
    let b = ray.direction.dot(&object_to_ray) * 2.0;
    let c = object_to_ray.dot(&object_to_ray) - 1.0;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        Intersections { values: Vec::new() }
    } else {
        let i1 = Intersection {
            obj: sphere,
            t: (-b - f64::sqrt(discriminant)) / (2.0 * a),
        };
        let i2 = Intersection {
            obj: sphere,
            t: (-b + f64::sqrt(discriminant)) / (2.0 * a),
        };
        Intersections {
            values: vec![i1, i2],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lighting::Material,
        object::{Object, ObjectType},
        transform,
        tuple::Tuple,
    };

    #[test]
    fn ray_sphere_normal_at() {
        assert_abs_diff_eq!(
            Object::SPHERE.normal_at(&Tuple::point(1.0, 0.0, 0.0)),
            Tuple::vector(1.0, 0.0, 0.0)
        );
        assert_abs_diff_eq!(
            Object::SPHERE.normal_at(&Tuple::point(0.0, 1.0, 0.0)),
            Tuple::vector(0.0, 1.0, 0.0)
        );
        assert_abs_diff_eq!(
            Object::SPHERE.normal_at(&Tuple::point(0.0, 0.0, 1.0)),
            Tuple::vector(0.0, 0.0, 1.0)
        );
        let n = Object::SPHERE.normal_at(&Tuple::point(
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
        ));
        assert_abs_diff_eq!(
            n,
            Tuple::vector(
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0
            )
        );
        assert_abs_diff_eq!(n.normalize(), n);
    }

    #[test]
    fn ray_sphere_normal_at_transformed() {
        let s1 = Object {
            object_type: ObjectType::Sphere,
            transform: transform::translation(0.0, 1.0, 0.0),
            material: Material::DEFAULT,
        };
        assert_abs_diff_eq!(
            s1.normal_at(&Tuple::point(0.0, 1.70711, -0.70711)),
            Tuple::vector(0.0, 0.70711, -0.70711)
        );

        let s2 = Object {
            object_type: ObjectType::Sphere,
            material: Material::DEFAULT,
            transform: transform::scaling(1.0, 0.5, 1.0)
                .mul_matrix(&transform::rotation_z(std::f64::consts::PI / 5.0)),
        };
        assert_abs_diff_eq!(
            s2.normal_at(&Tuple::point(
                0.0,
                f64::sqrt(2.0) / 2.0,
                -f64::sqrt(2.0) / 2.0
            )),
            Tuple::vector(0.0, 0.97014, -0.24254)
        );
    }
}
