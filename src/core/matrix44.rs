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
            self.m[0][0], self.m[1][0], self.m[2][0], self.m[3][0],
            self.m[0][1], self.m[1][1], self.m[2][1], self.m[3][1],
            self.m[0][2], self.m[1][2], self.m[2][2], self.m[3][2],
            self.m[0][3], self.m[1][3], self.m[2][3], self.m[3][3],
        )
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn mul(self, m: Self) -> Self {
        let mut r = Self::zero();

        for i in 0..4 {
            for j in 0..4 {
                r.m[i][j] = self.m[i][0] * m.m[0][j] +
                            self.m[i][1] * m.m[1][j] +
                            self.m[i][2] * m.m[2][j] +
                            self.m[i][3] * m.m[3][j]
            }
        }

        r
    }

    pub fn inverse(self) -> Self {
        panic!("Not implemented yet!");
    }
}
