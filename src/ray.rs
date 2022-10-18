use crate::{tuple::Tuple, matrix4::Matrix4, lighting::Material, world::WorldObject};
use approx::abs_diff_eq;

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray {
            origin: origin,
            direction: direction
        }
    }

    pub fn position(&self, t: f64) -> Tuple {
        self.origin.add(&self.direction.scalar_mul(t))
    }

    pub fn transform(&self, m: &Matrix4) -> Ray {
        Ray {
            origin: m.mul_tuple(&self.origin),
            direction: m.mul_tuple(&self.direction)
        }
    }
}

pub struct Sphere {
    transform:  Matrix4,
    pub material: Material
}

impl Sphere {

    pub const DEFAULT: Sphere =
        Sphere { transform: Matrix4::IDENTITY, material: Material::DEFAULT };

    pub const fn transform(self, transform: &Matrix4) -> Sphere {
        Sphere {
            transform: *transform,
            material: self.material
        }
    }

    pub const fn material(self, material: &Material) -> Sphere {
        Sphere {
            transform: self.transform,
            material: *material
        }
    }

}

impl WorldObject for Sphere  {
    fn material(&self) -> Material {
        self.material
    }

    fn transform(&self) -> Matrix4 {
        self.transform
    }

    fn object_normal(&self, local_point: &Tuple) -> Tuple {
        local_point.subtract(&Tuple::point(0.0, 0.0, 0.0)).normalize()
    }

    fn object_intersect(&self, ray: &Ray) -> Intersections<'_> {
        let object_to_ray = ray.origin.subtract(&Tuple::point(0.0, 0.0, 0.0));
        let a = ray.direction.dot(&ray.direction);
        let b = ray.direction.dot(&object_to_ray) * 2.0;
        let c = object_to_ray.dot(&object_to_ray) - 1.0;
        let discriminant  = b * b - 4.0 * a * c;
        
        if discriminant < 0.0 {
            Intersections { values: Vec::new() }
        } else {
            let i1 = Intersection { object: self, t: (-b - f64::sqrt(discriminant)) / (2.0 * a) };
            let i2 = Intersection { object: self, t: (-b + f64::sqrt(discriminant)) / (2.0 * a) };
            Intersections { values: vec!(i1, i2) }
        }
    }
}

#[derive(Copy, Clone)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a dyn WorldObject
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
    pub values: Vec<Intersection<'a>>
}

impl Intersections<'_> {
    pub fn hit(&self) -> Option<&Intersection> {
        self.values.iter().filter(|v| v.t >= 0.0).fold(None, |a_opt, b| {
            match a_opt {
                None => Some(b),
                Some(a) if a.t < b.t => Some(a),
                _ => Some(b)
            }
        })
    }

    pub fn intersect<'a>(object: &'a dyn WorldObject, ray: &Ray) -> Intersections<'a> {
        let object_ray = ray.transform(&object.transform().inverse());
        object.object_intersect(&object_ray)
    }
}

#[cfg(test)]
mod tests {
    use crate::{tuple::Tuple, transform, world::WorldObject};

    use super::{Ray, Sphere, Intersection, Intersections};

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
        let s = Sphere::DEFAULT;
        let xs = Intersections::intersect(&s, &ray);
        assert_eq!(xs.values.len(), 2);
        assert_abs_diff_eq!(xs.values[0].t, 4.0);
        assert_abs_diff_eq!(xs.values[1].t, 6.0);
    }

    #[test]
    fn sphere_intersect_one_point() {
        let ray = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::DEFAULT;
        let xs = Intersections::intersect(&s, &ray);
        assert_eq!(xs.values.len(), 2);
        assert_abs_diff_eq!(xs.values[0].t, 5.0);
        assert_abs_diff_eq!(xs.values[1].t, 5.0);
    }

    #[test]
    fn sphere_intersect_miss() {
        let ray = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::DEFAULT;
        let xs = Intersections::intersect(&s, &ray);
        assert_eq!(xs.values.len(), 0);
    }

    #[test]
    fn sphere_intersect_inside() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::DEFAULT;
        let xs = Intersections::intersect(&s, &ray);
        assert_eq!(xs.values.len(), 2);
        assert_abs_diff_eq!(xs.values[0].t, -1.0);
        assert_abs_diff_eq!(xs.values[1].t, 1.0);
    }

    #[test]
    fn sphere_intersect_behind() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::DEFAULT;
        let xs = Intersections::intersect(&s, &ray);
        assert_eq!(xs.values.len(), 2);
        assert_abs_diff_eq!(xs.values[0].t, -6.0);
        assert_abs_diff_eq!(xs.values[1].t, -4.0);
    }

    #[test]
    fn intersections_all_positive() {
        let s = Sphere::DEFAULT;
        let i1 = Intersection { object: &s, t: 1.0 };
        let i2= Intersection { object: &s, t: 2.0 };
        let xs = Intersections { values: vec!(i1, i2) };
        assert_eq!(xs.hit().map(|i| { i.t }), Some(i1.t));
    }

    #[test]
    fn intersections_negative() {
        let s = Sphere::DEFAULT;
        let i1 = Intersection { object: &s, t: -1.0 };
        let i2= Intersection { object: &s, t: 1.0 };
        let xs = Intersections { values: vec!(i2, i1) };
        assert_eq!(xs.hit().map(|i|{i.t}), Some(i2.t));
    }

    #[test]
    fn intersections_all_negative() {
        let s = Sphere::DEFAULT;
        let i1 = Intersection { object: &s, t: -2.0 };
        let i2= Intersection { object: &s, t: -1.0 };
        let xs = Intersections { values: vec!(i2, i1) };
        assert_eq!(xs.hit().map(|i|{i.t}), None);
    }

    #[test]
    fn intersections_smallest_non_negative() {
        let s = Sphere::DEFAULT;
        let i1 = Intersection { object: &s, t: 5.0 };
        let i2= Intersection { object: &s, t: 7.0 };
        let i3= Intersection { object: &s, t: -3.0 };
        let i4= Intersection { object: &s, t: 2.0 };
        let xs = Intersections { values: vec!(i1, i2, i3, i4) };
        assert_eq!(xs.hit().map(|i|{i.t}), Some(i4.t));
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
        let s = Sphere::DEFAULT.transform(&transform::scaling(2.0, 2.0, 2.0));
        let xs = Intersections::intersect(&s, &r);
        assert_eq!(xs.values.len(), 2);
        assert_abs_diff_eq!(xs.values[0].t, 3.0);
        assert_abs_diff_eq!(xs.values[1].t, 7.0);
    }

    #[test]
    fn ray_sphere_translated() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::DEFAULT.transform(&transform::translation(5.0, 0.0, 0.0));
        let xs = Intersections::intersect(&s, &r);
        assert_eq!(xs.values.len(), 0);
    }

    #[test]
    fn ray_sphere_normal_at() {
        let s = Sphere::DEFAULT;
        assert_abs_diff_eq!(<dyn WorldObject>::normal_at(&s, &Tuple::point(1.0, 0.0, 0.0)), Tuple::vector(1.0, 0.0, 0.0));
        assert_abs_diff_eq!(<dyn WorldObject>::normal_at(&s, &Tuple::point(0.0, 1.0, 0.0)), Tuple::vector(0.0,  1.0,  0.0));
        assert_abs_diff_eq!(<dyn WorldObject>::normal_at(&s, &Tuple::point(0.0, 0.0, 1.0)), Tuple::vector(0.0, 0.0, 1.0));
        let n = <dyn WorldObject>::normal_at(&s, &Tuple::point(f64::sqrt(3.0)/3.0, f64::sqrt(3.0)/3.0, f64::sqrt(3.0)/3.0));
        assert_abs_diff_eq!(n, Tuple::vector(f64::sqrt(3.0)/3.0, f64::sqrt(3.0)/3.0, f64::sqrt(3.0)/3.0));
        assert_abs_diff_eq!(n.normalize(), n);
    }

    #[test]
    fn ray_sphere_normal_at_transformed() {
        let s1 = Sphere::DEFAULT.transform(&transform::translation(0.0, 1.0, 0.0));
        assert_abs_diff_eq!(<dyn WorldObject>::normal_at(&s1, &Tuple::point(0.0, 1.70711, -0.70711)), Tuple::vector(0.0, 0.70711, -0.70711));

        let s2 = Sphere::DEFAULT.transform(&transform::scaling(1.0, 0.5, 1.0).mul_matrix(&transform::rotation_z(std::f64::consts::PI/5.0)));
        assert_abs_diff_eq!(<dyn WorldObject>::normal_at(&s2, &Tuple::point(0.0, f64::sqrt(2.0)/2.0, -f64::sqrt(2.0)/2.0)), Tuple::vector(0.0, 0.97014, -0.24254));
    }
}