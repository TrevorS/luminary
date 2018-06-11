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
