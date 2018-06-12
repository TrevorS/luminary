use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, Debug)]
pub struct Matrix44 {
    m: [[f64; 4]; 4],
}

impl Matrix44 {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn new(
        t00: f64, t01: f64, t02: f64, t03: f64,
        t10: f64, t11: f64, t12: f64, t13: f64,
        t20: f64, t21: f64, t22: f64, t23: f64,
        t30: f64, t31: f64, t32: f64, t33: f64,
    ) -> Self {
        Self {
            m: [
                [t00, t01, t02, t03],
                [t10, t11, t12, t13],
                [t20, t21, t22, t23],
                [t30, t31, t32, t33],
            ],
        }
    }

    pub fn new_from_array(m: [[f64; 4]; 4]) -> Self {
        Self { m: m }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn zero() -> Self {
        Self::new(
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
        )
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn identity() -> Self {
        Self::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn transpose(self) -> Self {
        Self::new(
            self[0][0], self[1][0], self[2][0], self[3][0],
            self[0][1], self[1][1], self[2][1], self[3][1],
            self[0][2], self[1][2], self[2][2], self[3][2],
            self[0][3], self[1][3], self[2][3], self[3][3],
        )
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn mul(self, m: Self) -> Self {
        let mut r = Self::zero();

        for i in 0..4 {
            for j in 0..4 {
                r.m[i][j] = self[i][0] * m[0][j] +
                            self[i][1] * m[1][j] +
                            self[i][2] * m[2][j] +
                            self[i][3] * m[3][j]
            }
        }

        r
    }

    pub fn inverse(&self) -> Self {
        let mut s = self.clone();
        let mut r = Self::identity();

        let identity = Self::identity();

        for i in 0..3 {
            let mut pivot = i;
            let mut pivot_size = s[i][i];

            if pivot_size < 0.0 {
                pivot_size = -pivot_size;

                for j in (i + 1)..4 {
                    let mut tmp = s[j][i];

                    if tmp < 0.0 {
                        tmp = -tmp;

                        if tmp > pivot_size {
                            pivot = j;
                            pivot_size = tmp;
                        }
                    }
                }
            }

            if pivot_size == 0.0 {
                // Cannot invert singular matrix
                return identity;
            }

            if pivot != i {
                for j in 0..4 {
                    let mut tmp = s[i][j];
                    s[i][j] = s[pivot][j];
                    s[pivot][j] = tmp;

                    tmp = r[i][j];
                    r[i][j] = r[pivot][j];
                    r[pivot][j] = tmp;
                }
            }

            for j in (i + 1)..4 {
                let f = s[j][i] / s[i][i];

                for k in 0..4 {
                    s[j][k] -= f * s[i][k];
                    r[j][k] -= f * r[i][k];
                }
            }
        }

        for i in (0..4).rev() {
            let mut f = s[i][i];

            if f == 0.0 {
                // Cannot invert singular matrix
                return identity;
            }

            for j in 0..4 {
                s[i][j] /= f;
                r[i][j] /= f;
            }

            for j in 0..i {
                f = s[j][i];

                for k in 0..4 {
                    s[j][k] -= f * s[i][k];
                    r[j][k] -= f * r[i][k];
                }
            }
        }

        r
    }
}

impl Index<usize> for Matrix44 {
    type Output = [f64; 4];

    fn index(&self, i: usize) -> &[f64; 4] {
        assert!(i <= 3);

        &self.m[i]
    }
}

impl IndexMut<usize> for Matrix44 {
    fn index_mut(&mut self, i: usize) -> &mut [f64; 4] {
        assert!(i <= 3);

        &mut self.m[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 0.00001;

    fn assert_float_value(expected: f64, value: f64) {
        assert!((value - expected).abs() < EPSILON);
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn assert_matrix_values(
        t00: f64, t01: f64, t02: f64, t03: f64,
        t10: f64, t11: f64, t12: f64, t13: f64,
        t20: f64, t21: f64, t22: f64, t23: f64,
        t30: f64, t31: f64, t32: f64, t33: f64,
        matrix: Matrix44,
    ) {
        assert_float_value(t00, matrix[0][0]);
        assert_float_value(t01, matrix[0][1]);
        assert_float_value(t02, matrix[0][2]);
        assert_float_value(t03, matrix[0][3]);
        assert_float_value(t10, matrix[1][0]);
        assert_float_value(t11, matrix[1][1]);
        assert_float_value(t12, matrix[1][2]);
        assert_float_value(t13, matrix[1][3]);
        assert_float_value(t20, matrix[2][0]);
        assert_float_value(t21, matrix[2][1]);
        assert_float_value(t22, matrix[2][2]);
        assert_float_value(t23, matrix[2][3]);
        assert_float_value(t30, matrix[3][0]);
        assert_float_value(t31, matrix[3][1]);
        assert_float_value(t32, matrix[3][2]);
        assert_float_value(t33, matrix[3][3]);
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    #[test]
    fn inverse() {
        let matrix = Matrix44::new(
            0.707107, 0.0, -0.707107, 0.0,
            -0.331295, 0.883452, -0.331295, 0.0,
            0.624695, 0.468521, 0.624695, 0.0,
            4.000574, 3.00043, 4.000574, 1.0,
        );

        let result = matrix.inverse();

        assert_matrix_values(
            0.707107, -0.331295, 0.624695, 0.0,
            0.0, 0.883452, 0.468521, 0.0,
            -0.707107, -0.331295, 0.624695, 0.0,
            0.0, 0.0, -6.404044, 1.0,
            result,
        );
    }
}
