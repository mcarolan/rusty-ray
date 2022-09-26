use crate::{tuple::Tuple, matrix4::Matrix4, lighting::Material};
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

type ShapeId = u32;

pub struct Sphere {
    pub id: ShapeId,
    transform:  Matrix4,
    pub material: Material
}

impl Sphere {
    pub fn new(id: ShapeId) -> Sphere {
        Sphere { id: id, transform: Matrix4::IDENTITY, material: Material::DEFAULT }
    }

    pub fn transform(self, transform: &Matrix4) -> Sphere {
        Sphere {
            id: self.id,
            transform: *transform,
            material: self.material
        }
    }

    pub fn material(self, material: &Material) -> Sphere {
        Sphere {
            id: self.id,
            transform: self.transform,
            material: *material
        }
    }

    pub fn normal_at(&self, world_point: &Tuple) -> Tuple {
        let object_point = self.transform.inverse().mul_tuple(&world_point);
        let object_normal = object_point.subtract(&Tuple::point(0.0, 0.0, 0.0));
        let world_normal = self.transform.inverse().transpose().mul_tuple(&object_normal);
        Tuple::vector(world_normal.x, world_normal.y, world_normal.z).normalize()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Intersection {
    pub id: ShapeId,
    pub t: f64
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && abs_diff_eq!(self.t, other.t)
    }
}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.t.partial_cmp(&other.t)
    }
}

pub struct Intersections {
    pub values: Vec<Intersection>
}

impl Intersections {
    pub fn hit(&self) -> Option<&Intersection> {
        self.values.iter().filter(|v| v.t >= 0.0).fold(None, |a_opt, b| {
            match a_opt {
                None => Some(b),
                Some(a) if a.t < b.t => Some(a),
                _ => Some(b)
            }
        })
    }
}

pub trait Intersect {
    fn intersect(&self, ray: &Ray) -> Intersections;
}

impl Intersect for Sphere {
    fn intersect(&self, ray: &Ray) -> Intersections {
        let ray2 = ray.transform(&self.transform.inverse());
        let sphere_to_ray = ray2.origin.subtract(&Tuple::point(0.0, 0.0, 0.0));
        let a = ray2.direction.dot(&ray2.direction);
        let b = ray2.direction.dot(&sphere_to_ray) * 2.0;
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant  = b * b - 4.0 * a * c;
        
        if discriminant < 0.0 {
            Intersections { values: Vec::new() }
        } else {
            let i1 = Intersection { id: self.id, t: (-b - f64::sqrt(discriminant)) / (2.0 * a) };
            let i2 = Intersection { id: self.id, t: (-b + f64::sqrt(discriminant)) / (2.0 * a) };
            Intersections { values: vec!(i1, i2) }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{tuple::Tuple, transform};

    use super::{Ray, Sphere, Intersect, Intersection, Intersections};

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
        let s = Sphere::new(0);
        let xs = s.intersect(&ray);
        assert_eq!(xs.values.len(), 2);
        assert_abs_diff_eq!(xs.values[0].t, 4.0);
        assert_abs_diff_eq!(xs.values[1].t, 6.0);
    }

    #[test]
    fn sphere_intersect_one_point() {
        let ray = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new(0);
        let xs = s.intersect(&ray);
        assert_eq!(xs.values.len(), 2);
        assert_abs_diff_eq!(xs.values[0].t, 5.0);
        assert_abs_diff_eq!(xs.values[1].t, 5.0);
    }

    #[test]
    fn sphere_intersect_miss() {
        let ray = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new(0);
        let xs = s.intersect(&ray);
        assert_eq!(xs.values.len(), 0);
    }

    #[test]
    fn sphere_intersect_inside() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new(0);
        let xs = s.intersect(&ray);
        assert_eq!(xs.values.len(), 2);
        assert_abs_diff_eq!(xs.values[0].t, -1.0);
        assert_abs_diff_eq!(xs.values[1].t, 1.0);
    }

    #[test]
    fn sphere_intersect_behind() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new(0);
        let xs = s.intersect(&ray);
        assert_eq!(xs.values.len(), 2);
        assert_abs_diff_eq!(xs.values[0].t, -6.0);
        assert_abs_diff_eq!(xs.values[1].t, -4.0);
    }

    #[test]
    fn intersections_all_positive() {
        let s = Sphere::new(0);
        let i1 = Intersection { t: 1.0, id: s.id };
        let i2= Intersection { t: 2.0, id: s.id };
        let xs = Intersections { values: vec!(i1, i2) };
        assert_eq!(xs.hit(), Some(&i1));
    }

    #[test]
    fn intersections_negative() {
        let s = Sphere::new(0);
        let i1 = Intersection { t: -1.0, id: s.id };
        let i2= Intersection { t: 1.0, id: s.id };
        let xs = Intersections { values: vec!(i2, i1) };
        assert_eq!(xs.hit(), Some(&i2));
    }

    #[test]
    fn intersections_all_negative() {
        let s = Sphere::new(0);
        let i1 = Intersection { t: -2.0, id: s.id };
        let i2= Intersection { t: -1.0, id: s.id };
        let xs = Intersections { values: vec!(i2, i1) };
        assert_eq!(xs.hit(), None);
    }

    #[test]
    fn intersections_smallest_non_negative() {
        let s = Sphere::new(0);
        let i1 = Intersection { t: 5.0, id: s.id };
        let i2= Intersection { t: 7.0, id: s.id };
        let i3= Intersection { t: -3.0, id: s.id };
        let i4= Intersection { t: 2.0, id: s.id };
        let xs = Intersections { values: vec!(i1, i2, i3, i4) };
        assert_eq!(xs.hit(), Some(&i4));
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
        let s = Sphere::new(0).transform(&transform::scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.values.len(), 2);
        assert_abs_diff_eq!(xs.values[0].t, 3.0);
        assert_abs_diff_eq!(xs.values[1].t, 7.0);
    }

    #[test]
    fn ray_sphere_translated() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new(0).transform(&transform::translation(5.0, 0.0, 0.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.values.len(), 0);
    }

    #[test]
    fn ray_sphere_normal_at() {
        let s = Sphere::new(0);
        assert_abs_diff_eq!(s.normal_at(&Tuple::point(1.0, 0.0, 0.0)), Tuple::vector(1.0, 0.0, 0.0));
        assert_abs_diff_eq!(s.normal_at(&Tuple::point(0.0, 1.0, 0.0)), Tuple::vector(0.0,  1.0,  0.0));
        assert_abs_diff_eq!(s.normal_at(&Tuple::point(0.0, 0.0, 1.0)), Tuple::vector(0.0, 0.0, 1.0));
        let n = s.normal_at(&Tuple::point(f64::sqrt(3.0)/3.0, f64::sqrt(3.0)/3.0, f64::sqrt(3.0)/3.0));
        assert_abs_diff_eq!(n, Tuple::vector(f64::sqrt(3.0)/3.0, f64::sqrt(3.0)/3.0, f64::sqrt(3.0)/3.0));
        assert_abs_diff_eq!(n.normalize(), n);
    }

    #[test]
    fn ray_sphere_normal_at_transformed() {
        let s1 = Sphere::new(0).transform(&transform::translation(0.0, 1.0, 0.0));
        assert_abs_diff_eq!(s1.normal_at(&Tuple::point(0.0, 1.70711, -0.70711)), Tuple::vector(0.0, 0.70711, -0.70711));

        let s2 = Sphere::new(1).transform(&transform::scaling(1.0, 0.5, 1.0).mul_matrix(&transform::rotation_z(std::f64::consts::PI/5.0)));
        assert_abs_diff_eq!(s2.normal_at(&Tuple::point(0.0, f64::sqrt(2.0)/2.0, -f64::sqrt(2.0)/2.0)), Tuple::vector(0.0, 0.97014, -0.24254));
    }
}