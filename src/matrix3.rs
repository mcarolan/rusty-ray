use std::fmt::Debug;

use approx::AbsDiffEq;

use crate::matrix2::Matrix2;

#[derive(PartialEq, Copy, Clone)]
pub struct Matrix3 {
    pub values: [f64; 9],
}

impl Matrix3 {
    pub fn builder(v1: f64, v2: f64, v3: f64) -> Matrix3Builder {
        let mut values = [-1.0; 9];

        values[0] = v1;
        values[1] = v2;
        values[2] = v3;

        Matrix3Builder {
            current: Box::new(Matrix3 { values: values }),
            offset: 3,
        }
    }

    pub fn row_col_index(r: usize, c: usize) -> usize {
        (r * 3) + c
    }

    pub fn sub_matrix(&self, row_to_remove: usize, col_to_remove: usize) -> Matrix2 {
        let mut values = [-1.0; 4];
        let mut r = 0;
        for cur_row in 0..3 {
            if cur_row == row_to_remove {
                continue;
            }
            let mut c = 0;
            for cur_col in 0..3 {
                if cur_col == col_to_remove {
                    continue;
                }
                values[Matrix2::row_col_index(r, c)] = self.values[Matrix3::row_col_index(cur_row, cur_col)];
                c = c + 1;
            }
            r = r + 1;
        }
        Matrix2 { values: values }
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
        for c in 0..3 {
            det = det + self.values[Matrix3::row_col_index(0, c)] * self.cofactor(0, c);
        }
        det
    }

    pub fn at(&self, r: usize, c: usize) -> f64 {
        self.values[Matrix3::row_col_index(r, c)]
    }
}

impl AbsDiffEq for Matrix3 {
    type Epsilon  = f64;

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

impl Debug for Matrix3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_value_width = self.values.iter().map(|f| { format!("{}", f).chars().count() }).max().unwrap_or(0);

        write!(f, "
        | {:^width$} | {:^width$} | {:^width$} |
        | {:^width$} | {:^width$} | {:^width$} |
        | {:^width$} | {:^width$} | {:^width$} |
        ",
        self.values[0], self.values[1], self.values[2],
        self.values[3], self.values[4], self.values[5], 
        self.values[6], self.values[7], self.values[8],
        width = max_value_width
        )   
    }
}

pub struct Matrix3Builder {
    current: Box<Matrix3>,
    offset: usize,
}

impl Matrix3Builder {
    pub fn row(self, v1: f64, v2: f64, v3: f64) -> Matrix3Builder2 {
        let Matrix3 { values: mut v } = *self.current;

        v[self.offset + 0] = v1;
        v[self.offset + 1] = v2;
        v[self.offset + 2] = v3;

        Matrix3Builder2 {
            current: Box::new(Matrix3 { values: v }),
            offset: self.offset + 3,
        }
    }
}

pub struct Matrix3Builder2 {
    current: Box<Matrix3>,
    offset: usize,
}

impl Matrix3Builder2 {
    pub fn row(self, v1: f64, v2: f64, v3: f64) -> Matrix3 {
        let Matrix3 { values: mut v } = *self.current;

        v[self.offset + 0] = v1;
        v[self.offset + 1] = v2;
        v[self.offset + 2] = v3;

        Matrix3 { values: v }
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix2::Matrix2;

    use super::Matrix3;

    #[test]
    fn matrix_construct_inspect() {
        let m = Matrix3
            ::builder(-3.0, 5.0, 0.0)
            .row(1.0, -2.0, -7.0)
            .row(0.0, 1.0,  1.0);

        assert_abs_diff_eq!(m.at(0, 0), -3.0);
        assert_abs_diff_eq!(m.at(1, 1), -2.0);
        assert_abs_diff_eq!(m.at(2, 2), 1.0);
    }

    #[test]
    fn matrix3_sub_matrix() {
        let a = Matrix3
            ::builder(1.0, 5.0, 0.0)
                 .row(-3.0, 2.0, 7.0)
                 .row(0.0, 6.0, -3.0);
        let expected = Matrix2
            ::builder(-3.0, 2.0)
                 .row(0.0, 6.0);

        assert_abs_diff_eq!(a.sub_matrix(0, 2), expected);
    }

    #[test]
    fn matrix3_minor() {
        let a = Matrix3
            ::builder(3.0, 5.0, 0.0)
                 .row(2.0, -1.0, -7.0)
                 .row(6.0, -1.0, 5.0);
        let b = a.sub_matrix(1, 0);

        assert_abs_diff_eq!(b.determinant(), 25.0);
        assert_abs_diff_eq!(a.minor(1, 0), 25.0)
    }

    #[test]
    fn matrix3_cofactor() {
        let a = Matrix3
            ::builder(3.0, 5.0, 0.0)
                 .row(2.0, -1.0, -7.0)
                 .row(6.0, -1.0, 5.0);

        assert_abs_diff_eq!(a.minor(0, 0), -12.0);
        assert_abs_diff_eq!(a.cofactor(0, 0), -12.0);
        assert_abs_diff_eq!(a.minor(1, 0), 25.0);
        assert_abs_diff_eq!(a.cofactor(1, 0), -25.0);
    }

    #[test]
    fn matrix3_determinant() {
        let a = Matrix3
            ::builder(1.0, 2.0, 6.0)
                 .row(-5.0, 8.0, -4.0)
                 .row(2.0, 6.0, 4.0);

        assert_abs_diff_eq!(a.cofactor(0, 0), 56.0);
        assert_abs_diff_eq!(a.cofactor(0, 1), 12.0);
        assert_abs_diff_eq!(a.cofactor(0, 2), -46.0);
        assert_abs_diff_eq!(a.determinant(), -196.0);
    }
}