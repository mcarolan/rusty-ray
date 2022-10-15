use std::fmt::Debug;

use approx::AbsDiffEq;
#[derive(PartialEq, Copy, Clone)]
pub struct Matrix2 {
    pub values: [f64; 4],
}

impl Matrix2 {
    #[allow(dead_code)]
    pub fn builder(v1: f64, v2: f64) -> Matrix2Builder {
        let mut values = [-1.0; 4];

        values[0] = v1;
        values[1] = v2;

        Matrix2Builder {
            current: Box::new(Matrix2 { values: values })
        }
    }

    pub fn row_col_index(r: usize, c: usize) -> usize {
        (r * 2) + c
    }

    pub fn determinant(&self) -> f64 {
        (self.values[0] * self.values[3]) - (self.values[1] * self.values[2])
    }

    #[allow(dead_code)]
    pub fn at(&self, r: usize, c: usize) -> f64 {
        self.values[(r * 2) + c]
    }
}

impl AbsDiffEq for Matrix2 {
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

impl Debug for Matrix2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_value_width = self.values.iter().map(|f| { format!("{}", f).chars().count() }).max().unwrap_or(0);

        write!(f, "
        | {:^width$} | {:^width$} |
        | {:^width$} | {:^width$} |
        ",
        self.values[0], self.values[1], 
        self.values[2], self.values[3],
        width = max_value_width
        )   
    }
}

pub struct Matrix2Builder {
    #[allow(dead_code)]
    current: Box<Matrix2>,
}

impl Matrix2Builder {
    #[allow(dead_code)]
    pub fn row(self, v1: f64, v2: f64) -> Matrix2 {
        let Matrix2 { values: mut v } = *self.current;

        v[2] = v1;
        v[3] = v2;

        Matrix2 { values: v }
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix2;

    #[test]
    fn matrix_construct_inspect()  {
        let m = Matrix2::builder(-3.0, 5.0).row(1.0, -2.0);
        assert_abs_diff_eq!(m.at(0, 0), -3.0);
        assert_abs_diff_eq!(m.at(0, 1), 5.0);
        assert_abs_diff_eq!(m.at(1, 0), 1.0);
        assert_abs_diff_eq!(m.at(1, 1), -2.0);
    }

    #[test]
    fn matrix_determinant() {
        let a = Matrix2::builder(1.0, 5.0).row(-3.0, 2.0);
        assert_abs_diff_eq!(a.determinant(), 17.0);
    }

}