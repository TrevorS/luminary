use core::matrix44::Matrix44;

#[derive(Clone, Copy, Debug)]
pub struct Transform {
    m: Matrix44,
    m_inv: Matrix44,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            m: Matrix44::identity(),
            m_inv: Matrix44::identity(),
        }
    }

    pub fn inverse(self) -> Self {
        Self {
            m: self.m_inv,
            m_inv: self.m,
        }
    }

    pub fn transponse(self) -> Self {
        Self {
            m: self.m.transpose(),
            m_inv: self.m_inv.transpose(),
        }
    }
}

impl From<[[f64; 4]; 4]> for Transform {
    fn from(array: [[f64; 4]; 4]) -> Self {
        let m = Matrix44::from(array);

        Self::from(m)
    }
}

impl From<Matrix44> for Transform {
    fn from(m: Matrix44) -> Self {
        Self {
            m,
            m_inv: m.inverse(),
        }
    }
}

impl From<(Matrix44, Matrix44)> for Transform {
    fn from((m, m_inv): (Matrix44, Matrix44)) -> Self {
        Self { m, m_inv }
    }
}
