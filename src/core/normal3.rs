use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub,
               SubAssign};

use core::utils::has_nans_3;
use core::value::Value;
use core::vector3::Vector3;

#[derive(Clone, Copy, Debug)]
pub struct Normal3<T: Value> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Value> Normal3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        assert!(!has_nans_3(x, y, z));

        Self { x: x, y: y, z: z }
    }

    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero(), T::zero())
    }

    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn abs_dot(self, other: Self) -> T {
        self.dot(other).abs()
    }

    pub fn length_squared(self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> T {
        self.length_squared().sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.length()
    }

    pub fn min_component(self) -> T {
        self.x.min(self.y.min(self.z))
    }

    pub fn max_component(self) -> T {
        self.x.max(self.y.max(self.z))
    }

    pub fn max_dimension(self) -> usize {
        if self.x > self.y {
            if self.x > self.z {
                0
            } else {
                2
            }
        } else {
            if self.y > self.z {
                1
            } else {
                2
            }
        }
    }

    pub fn min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }

    pub fn max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }

    pub fn permute(self, x: usize, y: usize, z: usize) -> Self {
        Self {
            x: self[x],
            y: self[y],
            z: self[z],
        }
    }

    pub fn face_forward(self, other: Vector3<T>) -> Self {
        let dot = self.dot(Self::from(other));

        if dot < T::zero() {
            -self
        } else {
            self
        }
    }
}

impl<T: Value> From<Vector3<T>> for Normal3<T> {
    fn from(v: Vector3<T>) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl<T: Value> Index<usize> for Normal3<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        assert!(i <= 2);

        match i {
            0 => &self.x,
            1 => &self.y,
            _ => &self.z,
        }
    }
}

impl<T: Value> IndexMut<usize> for Normal3<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        assert!(i <= 2);

        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => &mut self.z,
        }
    }
}

impl<T: Value> Add for Normal3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Value + AddAssign> AddAssign for Normal3<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T: Value> Sub for Normal3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Value + SubAssign> SubAssign for Normal3<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T: Value> Mul<T> for Normal3<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl<T: Value + MulAssign> MulAssign<T> for Normal3<T> {
    fn mul_assign(&mut self, other: T) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl<T: Value> Div<T> for Normal3<T> {
    type Output = Self;

    fn div(self, other: T) -> Self {
        let inv = T::one() / other;

        Self {
            x: self.x * inv,
            y: self.y * inv,
            z: self.z * inv,
        }
    }
}

impl<T: Value + MulAssign> DivAssign<T> for Normal3<T> {
    fn div_assign(&mut self, other: T) {
        let inv = T::one() / other;

        self.x *= inv;
        self.y *= inv;
        self.z *= inv;
    }
}

impl<T: Value> Neg for Normal3<T> {
    type Output = Self;

    fn neg(self) -> Self {
        let neg_one = T::one().neg();

        Self {
            x: neg_one * self.x,
            y: neg_one * self.y,
            z: neg_one * self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64;

    #[test]
    fn new() {
        let n = Normal3::new(1.0, 2.0, 3.0);

        assert_eq!(1.0, n.x);
        assert_eq!(2.0, n.y);
        assert_eq!(3.0, n.z);
    }

    #[test]
    fn new_zero() {
        let n: Normal3<f64> = Normal3::zero();

        assert_eq!(0.0, n.x);
        assert_eq!(0.0, n.y);
        assert_eq!(0.0, n.z);
    }

    #[test]
    fn new_zero_int() {
        let n: Normal3<i32> = Normal3::zero();

        assert_eq!(0, n.x);
        assert_eq!(0, n.y);
        assert_eq!(0, n.z);
    }

    #[test]
    #[should_panic]
    fn has_nans_true() {
        Normal3::new(1.0, 2.0, f64::NAN);
    }

    #[test]
    fn abs() {
        let n1 = Normal3::new(-1.0, -2.0, -3.0);
        let n2 = n1.abs();

        assert_eq!(1.0, n2.x);
        assert_eq!(2.0, n2.y);
        assert_eq!(3.0, n2.z);
    }

    #[test]
    fn dot() {
        let n1 = Normal3::new(1.0, 2.0, 3.0);
        let n2 = Normal3::new(1.0, 2.0, 3.0);

        let dot = n1.dot(n2);

        assert_eq!(14.0, dot);
    }

    #[test]
    fn abs_dot() {
        let n1 = Normal3::new(1.0, 2.0, 3.0);
        let n2 = Normal3::new(-1.0, -2.0, -3.0);

        let abs_dot = n1.abs_dot(n2);

        assert_eq!(14.0, abs_dot);
    }

    #[test]
    fn length_squared() {
        let n = Normal3::new(1.0, 2.0, 3.0);

        let length_squared = n.length_squared();

        assert_eq!(14.0, length_squared);
    }

    #[test]
    fn length() {
        let n = Normal3::new(1.0, 2.0, 3.0);

        let length = n.length();

        assert_eq!(3.7416573867739413, length);
    }

    #[test]
    fn normalize() {
        let n = Normal3::new(3.0, 1.0, 2.0);

        let normalized = n.normalize();

        assert_eq!(0.8017837257372732, normalized.x);
        assert_eq!(0.2672612419124244, normalized.y);
        assert_eq!(0.5345224838248488, normalized.z);
    }

    #[test]
    fn min_component() {
        let n = Normal3::new(1.0, 2.0, 3.0);

        let min_component = n.min_component();

        assert_eq!(1.0, min_component);
    }

    #[test]
    fn max_component() {
        let n = Normal3::new(1.0, 2.0, 3.0);

        let max_component = n.max_component();

        assert_eq!(3.0, max_component);
    }

    #[test]
    fn max_dimension() {
        let n = Normal3::new(1.0, 2.0, 3.0);

        let max_dimension = n.max_dimension();

        assert_eq!(2, max_dimension);
    }

    #[test]
    fn min() {
        let n1 = Normal3::new(1.0, 2.0, 3.0);
        let n2 = Normal3::new(3.0, 2.0, 1.0);

        let min = n1.min(n2);

        assert_eq!(1.0, min.x);
        assert_eq!(2.0, min.y);
        assert_eq!(1.0, min.z);
    }

    #[test]
    fn max() {
        let n1 = Normal3::new(1.0, 2.0, 3.0);
        let n2 = Normal3::new(3.0, 2.0, 1.0);

        let min = n1.max(n2);

        assert_eq!(3.0, min.x);
        assert_eq!(2.0, min.y);
        assert_eq!(3.0, min.z);
    }

    #[test]
    fn permute() {
        let n = Normal3::new(1.0, 2.0, 3.0);

        let permuted = n.permute(2, 0, 1);

        assert_eq!(3.0, permuted.x);
        assert_eq!(1.0, permuted.y);
        assert_eq!(2.0, permuted.z);
    }

    #[test]
    fn from_vector3() {
        let v3 = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let n3 = Normal3::from(v3);

        assert_eq!(1.0, n3.x);
        assert_eq!(2.0, n3.y);
        assert_eq!(3.0, n3.z);
    }

    #[test]
    fn index() {
        let n = Normal3::new(1.0, 2.0, 3.0);

        assert_eq!(1.0, n[0]);
        assert_eq!(2.0, n[1]);
        assert_eq!(3.0, n[2]);
    }

    #[test]
    fn index_mut() {
        let mut n = Normal3::new(1.0, 2.0, 3.0);
        n[0] = 5.0;

        assert_eq!(5.0, n[0]);
        assert_eq!(2.0, n[1]);
        assert_eq!(3.0, n[2]);
    }

    #[test]
    fn add() {
        let n1 = Normal3::new(1.0, 2.0, 3.0);
        let n2 = Normal3::new(1.0, 2.0, 3.0);

        let n3 = n1 + n2;

        assert_eq!(2.0, n3.x);
        assert_eq!(4.0, n3.y);
        assert_eq!(6.0, n3.z);
    }

    #[test]
    fn add_assign() {
        let mut n1 = Normal3::new(1.0, 2.0, 3.0);
        n1 += Normal3::new(1.0, 2.0, 3.0);

        assert_eq!(2.0, n1.x);
        assert_eq!(4.0, n1.y);
        assert_eq!(6.0, n1.z);
    }

    #[test]
    fn sub() {
        let n1 = Normal3::new(1.0, 2.0, 3.0);
        let n2 = Normal3::new(1.0, 2.0, 3.0);

        let n3 = n1 - n2;

        assert_eq!(0.0, n3.x);
        assert_eq!(0.0, n3.y);
        assert_eq!(0.0, n3.z);
    }

    #[test]
    fn sub_assign() {
        let mut n1 = Normal3::new(1.0, 2.0, 3.0);
        n1 -= Normal3::new(1.0, 2.0, 3.0);

        assert_eq!(0.0, n1.x);
        assert_eq!(0.0, n1.y);
        assert_eq!(0.0, n1.z);
    }

    #[test]
    fn mul() {
        let n1 = Normal3::new(1.0, 2.0, 3.0);
        let n2 = n1 * 2.0;

        assert_eq!(2.0, n2.x);
        assert_eq!(4.0, n2.y);
        assert_eq!(6.0, n2.z);
    }

    #[test]
    fn mul_assign() {
        let mut n = Normal3::new(1.0, 2.0, 3.0);
        n *= 2.0;

        assert_eq!(2.0, n.x);
        assert_eq!(4.0, n.y);
        assert_eq!(6.0, n.z);
    }

    #[test]
    fn div() {
        let n1 = Normal3::new(2.0, 4.0, 6.0);
        let n2 = n1 / 2.0;

        assert_eq!(1.0, n2.x);
        assert_eq!(2.0, n2.y);
        assert_eq!(3.0, n2.z);
    }

    #[test]
    fn div_assign() {
        let mut n1 = Normal3::new(2.0, 4.0, 6.0);
        n1 /= 2.0;

        assert_eq!(1.0, n1.x);
        assert_eq!(2.0, n1.y);
        assert_eq!(3.0, n1.z);
    }

    #[test]
    fn neg() {
        let n1 = -Normal3::new(2.0, 4.0, 6.0);

        assert_eq!(-2.0, n1.x);
        assert_eq!(-4.0, n1.y);
        assert_eq!(-6.0, n1.z);
    }
}
