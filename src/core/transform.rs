use std::ops::Mul;

use core::matrix44::Matrix44;
use core::transformable::Transformable;

use core::Vector3f;

#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub m: Matrix44,
    pub m_inv: Matrix44,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            m: Matrix44::identity(),
            m_inv: Matrix44::identity(),
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn translate(delta: Vector3f) -> Self {
        Self {
            m: Matrix44::new(
                1.0, 0.0, 0.0, delta.x,
                0.0, 1.0, 0.0, delta.y,
                0.0, 0.0, 1.0, delta.z,
                0.0, 0.0, 0.0, 1.0,
            ),
            m_inv: Matrix44::new(
                1.0, 0.0, 0.0, -delta.x,
                0.0, 1.0, 0.0, -delta.y,
                0.0, 0.0, 1.0, -delta.z,
                0.0, 0.0, 0.0, 1.0,
            )
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn scale(x: f64, y: f64, z: f64) -> Self {
        Self {
            m: Matrix44::new(
                x, 0.0, 0.0, 0.0,
                0.0, y, 0.0, 0.0,
                0.0, 0.0, z, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ),
            m_inv: Matrix44::new(
                1.0 / x, 0.0, 0.0, 0.0,
                0.0, 1.0 / y, 0.0, 0.0,
                0.0, 0.0, 1.0 / z, 0.0,
                0.0, 0.0, 0.0, 1.0,
            )
        }
    }

    pub fn inverse(self) -> Self {
        Self {
            m: self.m_inv,
            m_inv: self.m,
        }
    }

    pub fn transpose(self) -> Self {
        Self {
            m: self.m.transpose(),
            m_inv: self.m_inv.transpose(),
        }
    }

    pub fn transform<T: Transformable>(self, transformable: T) -> T {
        transformable.transform(self)
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

impl Mul for Transform {
    type Output = Self;

    fn mul(self, other: Transform) -> Self {
        Self {
            m: self.m.mul(other.m),
            m_inv: other.m.mul(self.m_inv),
        }
    }
}
