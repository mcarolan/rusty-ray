use std::fmt::Debug;

use approx::AbsDiffEq;

use crate::{tuple::Tuple, matrix3::Matrix3};

#[derive(PartialEq, Copy, Clone)]
pub struct Matrix4 {
    pub values: [f64; 16],
}

impl AbsDiffEq for Matrix4 {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        0.00001
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        let mut res = true;
        for i in 0..self.values.len() {
            res = res && f64::abs_diff_eq(&self.values[i], &other.values[i], epsilon);
        }
        res
    }
}

impl Debug for Matrix4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_value_width = self
            .values
            .iter()
            .map(|f| format!("{}", f).chars().count())
            .max()
            .unwrap_or(0);

        write!(
            f,
            "
        | {:^width$} | {:^width$} | {:^width$} | {:^width$} |
        | {:^width$} | {:^width$} | {:^width$} | {:^width$} |
        | {:^width$} | {:^width$} | {:^width$} | {:^width$} |
        | {:^width$} | {:^width$} | {:^width$} | {:^width$} |
        ",
            self.values[0],
            self.values[1],
            self.values[2],
            self.values[3],
            self.values[4],
            self.values[5],
            self.values[6],
            self.values[7],
            self.values[8],
            self.values[9],
            self.values[10],
            self.values[11],
            self.values[12],
            self.values[13],
            self.values[14],
            self.values[15],
            width = max_value_width
        )
    }
}

impl Matrix4 {
    pub const IDENTITY: Matrix4 = Matrix4 {
        values: [
            1.0, 0.0, 0.0, 0.0, 
            0.0, 1.0, 0.0, 0.0, 
            0.0, 0.0, 1.0, 0.0, 
            0.0, 0.0, 0.0, 1.0,
        ],
    };

    pub fn builder(v1: f64, v2: f64, v3: f64, v4: f64) -> Matrix4Builder {
        let mut values = [-1.0; 16];

        values[0] = v1;
        values[1] = v2;
        values[2] = v3;
        values[3] = v4;

        Matrix4Builder {
            current: Box::new(Matrix4 { values: values }),
            offset: 4,
        }
    }

    pub fn row_col_index(r: usize, c: usize) -> usize {
        (r * 4) + c
    }

    pub fn at(&self, r: usize, c: usize) -> f64 {
        self.values[Matrix4::row_col_index(r, c)]
    }

    pub fn mul_matrix(&self, other: &Matrix4) -> Matrix4 {
        let mut values = [-1.0; 16];
        for r in 0..4 {
            for c in 0..4 {
                values[Matrix4::row_col_index(r, c)] = self.values[Matrix4::row_col_index(r, 0)]
                    * other.values[Matrix4::row_col_index(0, c)]
                    + self.values[Matrix4::row_col_index(r, 1)]
                        * other.values[Matrix4::row_col_index(1, c)]
                    + self.values[Matrix4::row_col_index(r, 2)]
                        * other.values[Matrix4::row_col_index(2, c)]
                    + self.values[Matrix4::row_col_index(r, 3)]
                        * other.values[Matrix4::row_col_index(3, c)]
            }
        }
        Matrix4 { values: values }
    }

    pub fn mul_tuple(&self, other: &Tuple) -> Tuple {
        let mut values = [-1.0; 4];
        for i in 0..4 {
            values[i] = other.x * self.values[Matrix4::row_col_index(i, 0)]
                + other.y * self.values[Matrix4::row_col_index(i, 1)]
                + other.z * self.values[Matrix4::row_col_index(i, 2)]
                + other.w * self.values[Matrix4::row_col_index(i, 3)];
        }
        Tuple::new(values[0], values[1], values[2], values[3])
    }

    pub fn transpose(&self) -> Matrix4 {
        let mut values = [-1.0; 16];
        for r in 0..4 {
            for c in 0..4 {
                values[Matrix4::row_col_index(r, c)] = self.values[Matrix4::row_col_index(c, r)];
            }
        }
        Matrix4 { values: values }
    }

    pub fn sub_matrix(&self, row_to_remove: usize, col_to_remove: usize) -> Matrix3 {
        let mut values = [-1.0; 9];
        let mut r = 0;
        for cur_row in 0..4 {
            if cur_row == row_to_remove {
                continue;
            }
            let mut c = 0;
            for cur_col in 0..4 {
                if cur_col == col_to_remove {
                    continue;
                }
                values[Matrix3::row_col_index(r, c)] = self.values[Matrix4::row_col_index(cur_row, cur_col)];
                c = c + 1;
            }
            r = r + 1;
        }
        Matrix3 { values: values }
    }

    pub fn minor(&self, r: usize, c: usize) -> f64 {
        self.sub_matrix(r, c).determinant()
    }

    pub fn cofactor(&self, r: usize, c: usize) -> f64 {
        if (r + c) % 2 == 0 {
            self.minor(r, c)
        }
        else {
            -self.minor(r,  c)
        }
    }

    pub fn determinant(&self) -> f64 {
        let mut det = 0.0;
        for c in 0..4 {
            det = det + self.values[Matrix4::row_col_index(0, c)] * self.cofactor(0, c);
        }
        det
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Matrix4 {
        let determinant = self.determinant();

        if determinant == 0.0 {
            panic!("Matrix not invertible");
        }

        let mut values = [-1.0; 16];

        for row in 0..4 {
            for col in 0..4  {
                let c = self.cofactor(row, col);
                values[Matrix4::row_col_index(col, row)] = c / determinant;
            }
        }
        
        Matrix4 { values: values }
    }
}

pub struct Matrix4Builder {
    current: Box<Matrix4>,
    offset: usize,
}

impl Matrix4Builder {
    pub fn row(self, v1: f64, v2: f64, v3: f64, v4: f64) -> Matrix4Builder2 {
        let Matrix4 { values: mut v } = *self.current;

        v[self.offset + 0] = v1;
        v[self.offset + 1] = v2;
        v[self.offset + 2] = v3;
        v[self.offset + 3] = v4;

        Matrix4Builder2 {
            current: Box::new(Matrix4 { values: v }),
            offset: self.offset + 4,
        }
    }
}

pub struct Matrix4Builder2 {
    current: Box<Matrix4>,
    offset: usize,
}

impl Matrix4Builder2 {
    pub fn row(self, v1: f64, v2: f64, v3: f64, v4: f64) -> Matrix4Builder3 {
        let Matrix4 { values: mut v } = *self.current;

        v[self.offset + 0] = v1;
        v[self.offset + 1] = v2;
        v[self.offset + 2] = v3;
        v[self.offset + 3] = v4;

        Matrix4Builder3 {
            current: Box::new(Matrix4 { values: v }),
            offset: self.offset + 4,
        }
    }
}

pub struct Matrix4Builder3 {
    current: Box<Matrix4>,
    offset: usize,
}

impl Matrix4Builder3 {
    pub fn row(self, v1: f64, v2: f64, v3: f64, v4: f64) -> Matrix4 {
        let Matrix4 { values: mut v } = *self.current;

        v[self.offset + 0] = v1;
        v[self.offset + 1] = v2;
        v[self.offset + 2] = v3;
        v[self.offset + 3] = v4;

        Matrix4 { values: v }
    }
}

#[cfg(test)]
mod tests {
    use crate::{tuple::Tuple, matrix3::Matrix3};

    use super::Matrix4;

    #[test]
    fn matrix_construct_inspect() {
        let m = Matrix4::builder(1.0, 2.0, 3.0, 4.0)
            .row(5.5, 6.5, 7.5, 8.5)
            .row(9.0, 10.0, 11.0, 12.0)
            .row(13.5, 14.5, 15.5, 16.5);

        assert_abs_diff_eq!(m.at(0, 0), 1.0);
        assert_abs_diff_eq!(m.at(0, 3), 4.0);
        assert_abs_diff_eq!(m.at(1, 0), 5.5);
        assert_abs_diff_eq!(m.at(1, 2), 7.5);
        assert_abs_diff_eq!(m.at(2, 2), 11.0);
        assert_abs_diff_eq!(m.at(3, 0), 13.5);
        assert_abs_diff_eq!(m.at(3, 2), 15.5);
    }

    #[test]
    fn matrix_equality() {
        let m1 = Matrix4::builder(1.0, 2.0, 3.0, 4.0)
            .row(5.5, 6.5, 7.5, 8.5)
            .row(9.0, 10.0, 11.0, 12.0)
            .row(13.5, 14.5, 15.5, 16.5);

        let m2 = Matrix4::builder(1.0, 2.0, 3.0, 4.0)
            .row(5.5, 6.5, 7.5, 8.5)
            .row(9.0, 10.0, 11.0, 12.0)
            .row(13.5, 14.5, 15.5, 16.5);

        assert_eq!(m1, m2);
        assert!(m1 == m2);
    }

    #[test]
    fn matrix_inequality() {
        let m1 = Matrix4::builder(1.0, 2.0, 3.0, 4.0)
            .row(5.5, 6.5, 7.5, 8.5)
            .row(9.0, 10.0, 11.0, 12.0)
            .row(13.5, 14.5, 15.5, 16.5);

        let m2 = Matrix4::builder(1.0, 2.0, 3.0, 4.0)
            .row(5.5, 6.5, 7.0, 8.5)
            .row(9.0, 10.0, 11.0, 12.0)
            .row(13.5, 14.5, 15.5, 16.5);

        assert_ne!(m1, m2);
        assert!(m1 != m2);
    }

    #[test]
    fn matrix_multiplication() {
        let a = Matrix4::builder(1.0, 2.0, 3.0, 4.0)
            .row(5.0, 6.0, 7.0, 8.0)
            .row(9.0, 8.0, 7.0, 6.0)
            .row(5.0, 4.0, 3.0, 2.0);

        let b = Matrix4::builder(-2.0, 1.0, 2.0, 3.0)
            .row(3.0, 2.0, 1.0, -1.0)
            .row(4.0, 3.0, 6.0, 5.0)
            .row(1.0, 2.0, 7.0, 8.0);

        let expected = Matrix4::builder(20.0, 22.0, 50.0, 48.0)
            .row(44.0, 54.0, 114.0, 108.0)
            .row(40.0, 58.0, 110.0, 102.0)
            .row(16.0, 26.0, 46.0, 42.0);

        assert_abs_diff_eq!(a.mul_matrix(&b), expected);
    }

    #[test]
    fn matrix_tuple_multiplication() {
        let a = Matrix4::builder(1.0, 2.0, 3.0, 4.0)
            .row(2.0, 4.0, 4.0, 2.0)
            .row(8.0, 6.0, 4.0, 1.0)
            .row(0.0, 0.0, 0.0, 1.0);

        let b = Tuple::new(1.0, 2.0, 3.0, 1.0);

        assert_abs_diff_eq!(a.mul_tuple(&b), Tuple::new(18.0, 24.0, 33.0, 1.0));
    }

    #[test]
    fn matrix_multiplication_identity() {
        let a = Matrix4
            ::builder(0.0, 1.0, 2.0, 4.0)
                 .row(1.0, 2.0, 4.0, 8.0)
                 .row(2.0, 4.0, 8.0, 16.0)
                 .row(4.0, 8.0, 16.0, 32.0);
        assert_abs_diff_eq!(a.mul_matrix(&Matrix4::IDENTITY), a);
    }

    #[test]
    fn matrix_tuple_multiplication_identity() {
        let a = Tuple::new(1.0, 2.0, 3.0, 4.0);
        assert_abs_diff_eq!(Matrix4::IDENTITY.mul_tuple(&a), a);
    }

    #[test]
    fn matrix_transpose() {
        let a = Matrix4
            ::builder(0.0, 9.0, 3.0, 0.0)
                 .row(9.0, 8.0, 0.0, 8.0)
                 .row(1.0, 8.0, 5.0, 3.0)
                 .row(0.0, 0.0, 5.0, 8.0);

        let expected = Matrix4
            ::builder(0.0, 9.0, 1.0, 0.0)
                 .row(9.0, 8.0, 8.0, 0.0)
                 .row(3.0, 0.0, 5.0, 5.0)
                 .row(0.0, 8.0, 3.0, 8.0);

        assert_abs_diff_eq!(a.transpose(), expected);
    }

    #[test]
    fn matrix_transpose_identity() {
        assert_abs_diff_eq!(Matrix4::IDENTITY.transpose(), Matrix4::IDENTITY);
    }

    #[test]
    fn matrix4_sub_matrix() {
        let a = Matrix4
            ::builder(-6.0, 1.0, 1.0, 6.0)
                 .row(-8.0, 5.0, 8.0, 6.0)
                 .row(-1.0, 0.0, 8.0, 2.0)
                 .row(-7.0, 1.0, -1.0, 1.0);
        
        let expected = Matrix3
            ::builder(-6.0, 1.0, 6.0)
                 .row(-8.0, 8.0, 6.0)
                 .row(-7.0, -1.0, 1.0);

        assert_abs_diff_eq!(a.sub_matrix(2, 1), expected);
    }

    #[test]
    fn matrix4_determinant() {
        let a = Matrix4
            ::builder(-2.0, -8.0, 3.0, 5.0)
                 .row(-3.0, 1.0, 7.0, 3.0)
                 .row(1.0, 2.0, -9.0, 6.0)
                 .row(-6.0, 7.0, 7.0, -9.0);
        assert_abs_diff_eq!(a.cofactor(0, 0), 690.0);
        assert_abs_diff_eq!(a.cofactor(0, 1), 447.0);
        assert_abs_diff_eq!(a.cofactor(0, 2), 210.0);
        assert_abs_diff_eq!(a.cofactor(0, 3), 51.0);
        assert_abs_diff_eq!(a.determinant(), -4071.0);
    }

    #[test]
    fn matrix_is_invertible() {
        let a = Matrix4
            ::builder(6.0, 4.0, 4.0, 4.0)
                 .row(5.0, 5.0, 7.0, 6.0)
                 .row(4.0, -9.0, 3.0, -7.0)
                 .row(9.0, 1.0, 7.0, -6.0);

        assert_abs_diff_eq!(a.determinant(), -2120.0);
        assert!(a.is_invertible());
    }

    #[test]
    fn matrix_is_not_invertible() {
        let a = Matrix4
            ::builder(-4.0, 2.0, -2.0, -3.0)
                 .row(9.0, 6.0, 2.0, 6.0)
                 .row(0.0, -5.0, 1.0, -5.0)
                 .row(0.0, 0.0, 0.0, 0.0);

        assert_abs_diff_eq!(a.determinant(), 0.0);
        assert!(!a.is_invertible());
    }

    #[test]
    fn matrix_inverse() {
        let a = Matrix4
            ::builder(-5.0, 2.0, 6.0, -8.0)
                 .row(1.0, -5.0, 1.0, 8.0)
                 .row(7.0, 7.0, -6.0, -7.0)
                 .row(1.0, -3.0, 7.0, 4.0);
        let b = a.inverse();

        assert_abs_diff_eq!(a.determinant(), 532.0);
        assert_abs_diff_eq!(a.cofactor(2, 3), -160.0);
        assert_abs_diff_eq!(b.at(3, 2), -160.0 / 532.0);
        assert_abs_diff_eq!(a.cofactor(3, 2), 105.0);
        assert_abs_diff_eq!(b.at(2, 3), 105.0 / 532.0);

        let expected = Matrix4
            ::builder(0.21805, 0.45113, 0.24060, -0.04511)
                 .row(-0.80827, -1.45677, -0.44361, 0.52068)
                 .row(-0.07895, -0.22368, -0.05263, 0.19737)
                 .row(-0.52256, -0.81391, -0.30075, 0.30639);
        
        assert_abs_diff_eq!(b, expected);
    }

    #[test]
    fn matrix_inverse_2() {
        let a = Matrix4
            ::builder(8.0, -5.0, 9.0, 2.0)
                 .row(7.0, 5.0, 6.0, 1.0)
                 .row(-6.0, 0.0, 9.0, 6.0)
                 .row(-3.0, 0.0, -9.0, -4.0);
        
        let expected = Matrix4
            ::builder(-0.15385, -0.15385, -0.28205, -0.53846)
                 .row(-0.07692, 0.12308, 0.02564, 0.03077)
                 .row(0.35897, 0.35897, 0.43590, 0.92308)
                 .row(-0.69231, -0.69231, -0.76923, -1.92308);

        assert_abs_diff_eq!(a.inverse(), expected);
    }

    #[test]
    fn matrix_inverse_3() {
        let a = Matrix4
            ::builder(9.0, 3.0, 0.0, 9.0)
                 .row(-5.0, -2.0, -6.0, -3.0)
                 .row(-4.0, 9.0, 6.0, 4.0)
                 .row(-7.0, 6.0, 6.0, 2.0);

        let expected = Matrix4
            ::builder(-0.04074, -0.07778, 0.14444, -0.22222)
                 .row(-0.07778, 0.03333, 0.36667, -0.33333)
                 .row(-0.02901, -0.14630, -0.10926, 0.12963)
                 .row(0.17778, 0.06667, -0.26667, 0.33333);

        assert_abs_diff_eq!(a.inverse(), expected);
    }

    #[test]
    fn matrix_multiply_by_inverse() {
        let a = Matrix4
            ::builder(3.0, -9.0, 7.0, 3.0)
                 .row(3.0, -8.0, 2.0, -9.0)
                 .row(-4.0, 4.0, 4.0, 1.0)
                 .row(-6.0, 5.0, -1.0, 1.0);

        let b = Matrix4
            ::builder(8.0, 2.0, 2.0, 2.0)
                 .row(3.0, -1.0, 7.0, 0.0)
                 .row(7.0, 0.0, 5.0, 4.0)
                 .row(6.0, -2.0, 0.0, 5.0);

        let c = a.mul_matrix(&b);
        assert_abs_diff_eq!(c.mul_matrix(&b.inverse()), a)
    }
}
