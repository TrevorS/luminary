use std::cmp;

use std::ops::{
    IndexMut,
    Index,
    AddAssign,
    Add,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign,
    Neg,
};

use num::traits::{NumAssign, NumCast};
use num::traits::real::Real;

#[derive(Clone, Copy, Debug)]
pub struct Vector3<T: Real> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Real> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        let v = Vector3 { x: x, y: y, z: z };

        assert!(!v.has_nans());

        v
    }

    fn has_nans(&self) -> bool {
        &self.x != &self.x || &self.y != &self.y || &self.z != &self.z
    }

    fn abs(self) -> Self {
        Vector3{
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn abs_dot(self, other: Self) -> T {
        self.dot(other).abs()
    }

    fn cross(self, other: Self) -> Self {
        let v1x = self.x.to_f64().unwrap();
        let v1y = self.y.to_f64().unwrap();
        let v1z = self.z.to_f64().unwrap();

        let v2x = other.x.to_f64().unwrap();
        let v2y = other.y.to_f64().unwrap();
        let v2z = other.z.to_f64().unwrap();

        let x = NumCast::from((v1y * v2z) - (v1z * v2y)).unwrap();
        let y = NumCast::from((v1z * v2x) - (v1x * v2z)).unwrap();
        let z = NumCast::from((v1x * v2y) - (v1y * v2x)).unwrap();

        Vector3{
            x,
            y,
            z,
        }
    }

    fn length_squared(self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn length(self) -> T {
        self.length_squared().sqrt()
    }

    fn normalize(self) -> Self {
        self / self.length()
    }
}

impl<T: Real> Index<usize> for Vector3<T> {
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

impl<T: Real> IndexMut<usize> for Vector3<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        assert!(i <= 2);

        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => &mut self.z,
        }
    }
}

impl<T: Real> Add for Vector3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector3{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Real + NumAssign> AddAssign for Vector3<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T: Real> Sub for Vector3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector3{
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Real + SubAssign> SubAssign for Vector3<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T: Real> Mul<T> for Vector3<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Vector3{
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl<T: Real + MulAssign> MulAssign<T> for Vector3<T> {
    fn mul_assign(&mut self, other: T) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl<T: Real> Div<T> for Vector3<T> {
    type Output = Self;

    fn div(self, other: T) -> Self {
        let one: T = NumCast::from(1usize).unwrap();
        let inv = one / other;

        Vector3{
            x: self.x * inv,
            y: self.y * inv,
            z: self.z * inv,
        }
    }
}

impl<T: Real + MulAssign> DivAssign<T> for Vector3<T> {
    fn div_assign(&mut self, other: T) {
        let one: T = NumCast::from(1usize).unwrap();
        let inv = one / other;

        self.x *= inv;
        self.y *= inv;
        self.z *= inv;
    }
}

impl<T: Real> Neg for Vector3<T> {
    type Output = Self;

    fn neg(self) -> Self {
        let neg_one: T = NumCast::from(-1isize).unwrap();

        Vector3{
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
        let v = Vector3::new(1.0, 2.0, 3.0);

        assert_eq!(1.0, v.x);
        assert_eq!(2.0, v.y);
        assert_eq!(3.0, v.z);
    }

    #[test]
    #[should_panic]
    fn has_nans_true() {
        Vector3::new(1.0, 2.0, f64::NAN);
    }

    #[test]
    fn has_nans_false() {
        let v = Vector3::new(1.0, 2.0, 3.0);

        assert_eq!(false, v.has_nans())
    }

    #[test]
    fn abs() {
        let v1 = Vector3::new(-1.0, -2.0, -3.0);
        let v2 = v1.abs();

        assert_eq!(1.0, v2.x);
        assert_eq!(2.0, v2.y);
        assert_eq!(3.0, v2.z);
    }

    #[test]
    fn dot() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(1.0, 2.0, 3.0);

        let dot = v1.dot(v2);

        assert_eq!(14.0, dot);
    }

    #[test]
    fn abs_dot() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(-1.0, -2.0, -3.0);

        let abs_dot = v1.abs_dot(v2);

        assert_eq!(14.0, abs_dot);
    }

    #[test]
    fn cross() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);

        let v3 = v1.cross(v2);

        assert_eq!(-3.0, v3.x);
        assert_eq!(6.0, v3.y);
        assert_eq!(-3.0, v3.z);
    }

    #[test]
    fn length_squared() {
        let v = Vector3::new(1.0, 2.0, 3.0);

        let length_squared = v.length_squared();

        assert_eq!(14.0, length_squared);
    }

    #[test]
    fn length() {
        let v = Vector3::new(1.0, 2.0, 3.0);

        let length = v.length();

        assert_eq!(3.7416573867739413, length);
    }

    #[test]
    fn normalize() {
        let v = Vector3::new(3.0, 1.0, 2.0);

        let normalized = v.normalize();

        assert_eq!(0.8017837257372732, normalized.x);
        assert_eq!(0.2672612419124244, normalized.y);
        assert_eq!(0.5345224838248488, normalized.z);
    }

    #[test]
    fn index() {
        let v = Vector3::new(1.0, 2.0, 3.0);

        assert_eq!(1.0, v[0]);
        assert_eq!(2.0, v[1]);
        assert_eq!(3.0, v[2]);
    }

    #[test]
    fn index_mut() {
        let mut v = Vector3::new(1.0, 2.0, 3.0);
        v[0] = 5.0;

        assert_eq!(5.0, v[0]);
        assert_eq!(2.0, v[1]);
        assert_eq!(3.0, v[2]);
    }

    #[test]
    fn plus() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(1.0, 2.0, 3.0);

        let v3 = v1 + v2;

        assert_eq!(2.0, v3.x);
        assert_eq!(4.0, v3.y);
        assert_eq!(6.0, v3.z);
    }

    #[test]
    fn plus_assign() {
        let mut v1 = Vector3::new(1.0, 2.0, 3.0);
        v1 += Vector3::new(1.0, 2.0, 3.0);

        assert_eq!(2.0, v1.x);
        assert_eq!(4.0, v1.y);
        assert_eq!(6.0, v1.z);
    }

    #[test]
    fn sub() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(1.0, 2.0, 3.0);

        let v3 = v1 - v2;

        assert_eq!(0.0, v3.x);
        assert_eq!(0.0, v3.y);
        assert_eq!(0.0, v3.z);
    }

    #[test]
    fn sub_assign() {
        let mut v1 = Vector3::new(1.0, 2.0, 3.0);
        v1 -= Vector3::new(1.0, 2.0, 3.0);

        assert_eq!(0.0, v1.x);
        assert_eq!(0.0, v1.y);
        assert_eq!(0.0, v1.z);
    }

    #[test]
    fn mul() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = v1 * 2.0;

        assert_eq!(2.0, v2.x);
        assert_eq!(4.0, v2.y);
        assert_eq!(6.0, v2.z);
    }

    #[test]
    fn mul_assign() {
        let mut v1 = Vector3::new(1.0, 2.0, 3.0);
        v1 *= 2.0;

        assert_eq!(2.0, v1.x);
        assert_eq!(4.0, v1.y);
        assert_eq!(6.0, v1.z);
    }

    #[test]
    fn div() {
        let v1 = Vector3::new(2.0, 4.0, 6.0);
        let v2 = v1 / 2.0;

        assert_eq!(1.0, v2.x);
        assert_eq!(2.0, v2.y);
        assert_eq!(3.0, v2.z);
    }

    #[test]
    fn div_assign() {
        let mut v1 = Vector3::new(2.0, 4.0, 6.0);
        v1 /= 2.0;

        assert_eq!(1.0, v1.x);
        assert_eq!(2.0, v1.y);
        assert_eq!(3.0, v1.z);
    }

    #[test]
    fn neg() {
        let v1 = -Vector3::new(2.0, 4.0, 6.0);

        assert_eq!(-2.0, v1.x);
        assert_eq!(-4.0, v1.y);
        assert_eq!(-6.0, v1.z);
    }
}
