use crate::matrix4::Matrix4;

pub fn translation(x: f64, y: f64, z: f64) -> Matrix4 {
    Matrix4::builder(1.0, 0.0, 0.0, x)
        .row(0.0, 1.0, 0.0, y)
        .row(0.0, 0.0, 1.0, z)
        .row(0.0, 0.0, 0.0, 1.0)
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix4 {
    Matrix4::builder(x, 0.0, 0.0, 0.0)
        .row(0.0, y, 0.0, 0.0)
        .row(0.0, 0.0, z, 0.0)
        .row(0.0, 0.0, 0.0, 1.0)
}

pub fn rotation_x(rads: f64) -> Matrix4 {
    Matrix4::builder(1.0, 0.0, 0.0, 0.0)
        .row(0.0, f64::cos(rads), -f64::sin(rads), 0.0)
        .row(0.0, f64::sin(rads), f64::cos(rads), 0.0)
        .row(0.0, 0.0, 0.0, 1.0)
}

pub fn rotation_y(rads: f64) -> Matrix4 {
    Matrix4::builder(f64::cos(rads), 0.0, f64::sin(rads), 0.0)
        .row(0.0, 1.0, 0.0, 0.0)
        .row(-f64::sin(rads), 0.0, f64::cos(rads), 0.0)
        .row(0.0, 0.0, 0.0, 1.0)
}

pub fn rotation_z(rads: f64) -> Matrix4 {
    Matrix4::builder(f64::cos(rads), -f64::sin(rads), 0.0, 0.0)
        .row(f64::sin(rads), f64::cos(rads), 0.0, 0.0)
        .row(0.0, 0.0, 1.0, 0.0)
        .row(0.0, 0.0, 0.0, 1.0)
}

pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix4 {
    Matrix4::builder(1.0, xy, xz, 0.0)
        .row(yx, 1.0, yz, 0.0)
        .row(zx, zy, 1.0, 0.0)
        .row(0.0, 0.0, 0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::tuple::Tuple;

    use super::*;

    #[test]
    fn transform_translate() {
        let a = translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);
        assert_abs_diff_eq!(a.mul_tuple(&p), Tuple::point(2.0, 1.0, 7.0));
    }

    #[test]
    fn transform_inverse_translate() {
        let a = translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);
        assert_abs_diff_eq!(a.inverse().mul_tuple(&p), Tuple::point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn transform_translate_vector() {
        let a = translation(5.0, -3.0, 2.0);
        let v = Tuple::vector(-3.0, 4.0, 5.0);
        assert_abs_diff_eq!(a.mul_tuple(&v), v);
    }

    #[test]
    fn transform_scale_point() {
        let a = scaling(2.0, 3.0, 4.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);
        assert_abs_diff_eq!(a.mul_tuple(&p), Tuple::point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn transform_scale_vector() {
        let a = scaling(2.0, 3.0, 4.0);
        let p = Tuple::vector(-4.0, 6.0, 8.0);
        assert_abs_diff_eq!(a.mul_tuple(&p), Tuple::vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn transform_inverse_scale() {
        let a = scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);
        assert_abs_diff_eq!(a.inverse().mul_tuple(&v), Tuple::vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn transform_reflection() {
        let a = scaling(-1.0, 1.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_abs_diff_eq!(a.mul_tuple(&p), Tuple::point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn transform_rotate_x() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);

        assert_abs_diff_eq!(
            half_quarter.mul_tuple(&p),
            Tuple::point(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0)
        );
        assert_abs_diff_eq!(full_quarter.mul_tuple(&p), Tuple::point(0.0, 0.0, 1.0));
    }

    #[test]
    fn transform_rotate_y() {
        let p = Tuple::point(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);
        assert_abs_diff_eq!(
            half_quarter.mul_tuple(&p),
            Tuple::point(f64::sqrt(2.0) / 2.0, 0.0, f64::sqrt(2.0) / 2.0)
        );
        assert_abs_diff_eq!(full_quarter.mul_tuple(&p), Tuple::point(1.0, 0.0, 0.0));
    }

    #[test]
    fn transform_rotate_z() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);
        assert_abs_diff_eq!(
            half_quarter.mul_tuple(&p),
            Tuple::point(-(f64::sqrt(2.0) / 2.0), f64::sqrt(2.0) / 2.0, 0.0)
        );
        assert_abs_diff_eq!(full_quarter.mul_tuple(&p), Tuple::point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn transform_shearing() {
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_abs_diff_eq!(
            shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0).mul_tuple(&p),
            Tuple::point(6.0, 3.0, 4.0)
        );
        assert_abs_diff_eq!(
            shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0).mul_tuple(&p),
            Tuple::point(2.0, 5.0, 4.0)
        );
        assert_abs_diff_eq!(
            shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0).mul_tuple(&p),
            Tuple::point(2.0, 7.0, 4.0)
        );
        assert_abs_diff_eq!(
            shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0).mul_tuple(&p),
            Tuple::point(2.0, 3.0, 6.0)
        );
        assert_abs_diff_eq!(
            shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0).mul_tuple(&p),
            Tuple::point(2.0, 3.0, 7.0)
        );
    }

    #[test]
    fn transform_chain() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        let p2 = a.mul_tuple(&p);
        assert_abs_diff_eq!(p2, Tuple::point(1.0, -1.0, 0.0));
        let p3 = b.mul_tuple(&p2);
        assert_abs_diff_eq!(p3, Tuple::point(5.0, -5.0, 0.0));
        let p4 = c.mul_tuple(&p3);
        assert_abs_diff_eq!(p4, Tuple::point(15.0, 0.0, 7.0));

        let t = c.mul_matrix(&b).mul_matrix(&a);
        assert_abs_diff_eq!(t.mul_tuple(&p), Tuple::point(15.0, 0.0, 7.0));
    }
}
