use approx::AbsDiffEq;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl AbsDiffEq for Tuple
{
    type Epsilon  = f64;

    fn default_epsilon() -> Self::Epsilon {
        0.00001
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        f64::abs_diff_eq(&self.x, &other.x, epsilon) &&
        f64::abs_diff_eq(&self.y, &other.y, epsilon) &&
        f64::abs_diff_eq(&self.z, &other.z, epsilon) &&
        f64::abs_diff_eq(&self.w, &other.w, epsilon)
    }
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple {
            x: x,
            y: y,
            z: z,
            w: 1.0
        }
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Tuple  {
        Tuple {
            x: x,
            y: y,
            z: z,
            w: 0.0
        }
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        !self.is_point()
    }

    pub fn add(&self, other: &Tuple) -> Tuple {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w
        }
    }

    pub fn subtract(&self, other: &Tuple) -> Tuple {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w
        }
    }

    pub fn negate(&self) -> Tuple {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w
        }
    }

    pub fn scalar_mul(&self, n: f64) -> Tuple {
        Tuple {
            x: self.x * n,
            y: self.y * n,
            z: self.z * n,
            w: self.w * n
        }
    }

    pub fn scalar_div(&self, n: f64) -> Tuple {
        Tuple {
            x: self.x / n,
            y: self.y / n,
            z: self.z / n,
            w: self.w / n
        }
    }

    pub fn magnitude(&self) -> f64 {
        f64::sqrt((self.x * self.x)  + (self.y * self.y) + (self.z * self.z) + (self.w  * self.w))
    }

    pub fn normalize(&self) -> Tuple {
        let mag  = self.magnitude();
        self.scalar_div(mag)
    }

    pub fn dot(&self, other: &Tuple) -> f64 {
        (self.x * other.x) +
        (self.y * other.y) +
        (self.z * other.z) +
        (self.w * other.w)
    }

    pub fn cross(&self, other: &Tuple) -> Tuple {
        Tuple::vector(
            self.y * other.z - self.z  * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::Tuple;

    #[test]
    fn test_point_tuple() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);

        assert_abs_diff_eq!(a.x, 4.3);
        assert_abs_diff_eq!(a.y, -4.2);
        assert_abs_diff_eq!(a.z, 3.1);
        assert_abs_diff_eq!(a.w, 1.0);
        assert_eq!(a.is_point(), true);

        let b = Tuple::point(4.0, -4.0, 3.0);

        assert_abs_diff_eq!(b, Tuple::new(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn test_vector_tuple() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);

        assert_relative_eq!(a.x, 4.3);
        assert_relative_eq!(a.y, -4.2);
        assert_relative_eq!(a.z, 3.1);
        assert_abs_diff_eq!(a.w, 0.0);
        assert_eq!(a.is_vector(), true);

        let b = Tuple::vector(4.0, -4.0, 3.0);
        assert_abs_diff_eq!(b, Tuple::new(4.0, -4.0, 3.0, 0.0));
    }

    #[test]
    fn test_add_tuples() {
        let a1 = Tuple  { x: 3.0, y: -2.0, z: 5.0, w: 1.0 };
        let a2 = Tuple { x: -2.0, y: 3.0, z: 1.0, w: 0.0 };

        assert_abs_diff_eq!(a1.add(&a2), Tuple { x: 1.0, y: 1.0, z: 6.0, w: 1.0  });
    }

    #[test]
    fn test_subtract_two_points() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let p2 = Tuple::point(5.0, 6.0, 7.0);
        assert_abs_diff_eq!(p1.subtract(&p2), Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn test_subtract_vector_point() {
        let p = Tuple::point(3.0, 2.0, 1.0);
        let v = Tuple::vector(5.0, 6.0, 7.0);
        assert_abs_diff_eq!(p.subtract(&v), Tuple::point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn test_subtract_two_vectors() {
        let v1 = Tuple::vector(3.0, 2.0, 1.0);
        let v2 = Tuple::vector(5.0, 6.0, 7.0);

        assert_abs_diff_eq!(v1.subtract(&v2), Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn test_subtract_vector_from_zero_vector() {
        let zero = Tuple::vector(0.0, 0.0, 0.0);
        let v = Tuple::vector(1.0, -2.0, 3.0);
        assert_abs_diff_eq!(zero.subtract(&v), Tuple::vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn test_negate() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_abs_diff_eq!(a.negate(), Tuple::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn test_scalar_mul() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_abs_diff_eq!(a.scalar_mul(3.5), Tuple::new(3.5, -7.0, 10.5, -14.0));
        assert_abs_diff_eq!(a.scalar_mul(0.5), Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn test_scalar_div() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_abs_diff_eq!(a.scalar_div(2.0), Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn test_magnitude() {
        assert_abs_diff_eq!(Tuple::vector(1.0, 0.0, 0.0).magnitude(), 1.0);
        assert_abs_diff_eq!(Tuple::vector(0.0, 1.0, 0.0).magnitude(), 1.0);
        assert_abs_diff_eq!(Tuple::vector(0.0, 0.0, 1.0).magnitude(), 1.0);
        assert_abs_diff_eq!(Tuple::vector(1.0, 2.0, 3.0).magnitude(), f64::sqrt(14.0));
        assert_abs_diff_eq!(Tuple::vector(-1.0, -2.0, -3.0).magnitude(), f64::sqrt(14.0));
    }

    #[test]
    fn test_normalize() {
        assert_abs_diff_eq!(Tuple::vector(4.0, 0.0, 0.0).normalize(), Tuple::vector(1.0, 0.0, 0.0));
        assert_abs_diff_eq!(Tuple::vector(1.0, 2.0, 3.0).normalize(), Tuple::vector(0.26726, 0.53452, 0.80178));
        assert_abs_diff_eq!(Tuple::vector(1.0, 2.0, 3.0).normalize().magnitude(), 1.0);
    }

    #[test]
    fn test_dot() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        assert_abs_diff_eq!(a.dot(&b), 20.0);
    }

    #[test]
    fn test_cross() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        assert_abs_diff_eq!(a.cross(&b), Tuple::vector(-1.0, 2.0, -1.0));
        assert_abs_diff_eq!(b.cross(&a), Tuple::vector(1.0, -2.0, 1.0));
    }
}
