use std::ops::{
    Index,
    IndexMut,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
};

use num::NumCast;

use core::utils::has_nans_3;
use core::value::Value;
use core::vector3::Vector3;

#[derive(Clone, Copy, Debug)]
pub struct Point3<T: Value> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Value> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        assert!(!has_nans_3(x, y, z));

        Self{ x: x, y: y, z: z }
    }

    pub fn zero() -> Self {
        Self::new(
            T::zero(),
            T::zero(),
            T::zero(),
        )
    }

    pub fn distance_squared(self, other: Self) -> T {
        (self - other).length_squared()
    }

    pub fn distance(self, other: Self) -> T {
        (self - other).length()
    }

    pub fn lerp(self, other: Self, t: f64) -> Self {
        // What should this return for int based Points?
        (self * (T::one() - NumCast::from(t).unwrap())) +
            (other * NumCast::from(t).unwrap())
    }

    pub fn min(self, other: Self) -> Self {
        Self{
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }

    pub fn max(self, other: Self) -> Self {
        Self{
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }

    pub fn floor(self) -> Self {
        Self{
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor(),
        }
    }

    pub fn ceil(self) -> Self {
        Self{
            x: self.x.ceil(),
            y: self.y.ceil(),
            z: self.z.ceil(),
        }
    }

    pub fn abs(self) -> Self {
        Self{
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    pub fn permute(self, x: usize, y: usize, z: usize) -> Self {
        Self{
            x: self[x],
            y: self[y],
            z: self[z],
        }
    }
}

impl<T: Value> From<Vector3<T>> for Point3<T> {
    fn from(v: Vector3<T>) -> Self {
        Self{
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl<T: Value> Index<usize> for Point3<T> {
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

impl<T: Value> IndexMut<usize> for Point3<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        assert!(i <= 2);

        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => &mut self.z,
        }
    }
}

impl<T: Value> Add for Point3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Value> Add<Vector3<T>> for Point3<T> {
    type Output = Self;

    fn add(self, other: Vector3<T>) -> Self {
        Self{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Value + AddAssign> AddAssign<Vector3<T>> for Point3<T> {
    fn add_assign(&mut self, other: Vector3<T>) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T: Value> Sub<Self> for Point3<T> {
    type Output = Vector3<T>;

    fn sub(self, other: Self) -> Vector3<T> {
        Vector3{
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Value> Sub<Vector3<T>> for Point3<T> {
    type Output = Self;

    fn sub(self, other: Vector3<T>) -> Self {
        Self{
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Value + SubAssign> SubAssign<Vector3<T>> for Point3<T> {
    fn sub_assign(&mut self, other: Vector3<T>) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T: Value> Mul<T> for Point3<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Self{
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl<T: Value + MulAssign> MulAssign<T> for Point3<T> {
    fn mul_assign(&mut self, other: T) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64;

    #[test]
    fn new() {
        let p = Point3::new(1.0, 2.0, 3.0);

        assert_eq!(1.0, p.x);
        assert_eq!(2.0, p.y);
        assert_eq!(3.0, p.z);
    }

    #[test]
    fn new_int() {
        let p = Point3::new(1, 2, 3);

        assert_eq!(1, p.x);
        assert_eq!(2, p.y);
        assert_eq!(3, p.z);
    }

    #[test]
    fn new_zero() {
        let p: Point3<f64> = Point3::zero();

        assert_eq!(0.0, p.x);
        assert_eq!(0.0, p.y);
        assert_eq!(0.0, p.z);
    }

    #[test]
    fn new_zero_int() {
        let p: Point3<i32> = Point3::zero();

        assert_eq!(0, p.x);
        assert_eq!(0, p.y);
        assert_eq!(0, p.z);
    }

    #[test]
    #[should_panic]
    fn has_nans_true() {
        Point3::new(1.0, 2.0, f64::NAN);
    }

    #[test]
    fn distance() {
        let p1 = Point3::new(2.0, 4.0, 6.0);
        let p2 = Point3::new(1.0, 2.0, 3.0);

        let distance = p1.distance(p2);

        assert_eq!(3.7416573867739413, distance);
    }

    #[test]
    fn distance_squared() {
        let p1 = Point3::new(2.0, 4.0, 6.0);
        let p2 = Point3::new(1.0, 2.0, 3.0);

        let distance_squared = p1.distance_squared(p2);

        assert_eq!(14.0, distance_squared);
    }

    #[test]
    fn lerp() {
        let p1 = Point3::new(2.0, 2.0, 2.0);
        let p2 = Point3::new(6.0, 6.0, 6.0);

        let lerp = p1.lerp(p2, 0.5);

        assert_eq!(4.0, lerp.x);
        assert_eq!(4.0, lerp.y);
        assert_eq!(4.0, lerp.z);
    }

    #[test]
    fn min() {
        let p1 = Point3::new(1.0, 2.0, 3.0);
        let p2 = Point3::new(3.0, 2.0, 1.0);

        let min = p1.min(p2);

        assert_eq!(1.0, min.x);
        assert_eq!(2.0, min.y);
        assert_eq!(1.0, min.z);
    }

    #[test]
    fn max() {
        let p1 = Point3::new(1.0, 2.0, 3.0);
        let p2 = Point3::new(3.0, 2.0, 1.0);

        let min = p1.max(p2);

        assert_eq!(3.0, min.x);
        assert_eq!(2.0, min.y);
        assert_eq!(3.0, min.z);
    }

    #[test]
    fn floor() {
        let p = Point3::new(1.1, 2.2, 3.3);

        let floor = p.floor();

        assert_eq!(1.0, floor.x);
        assert_eq!(2.0, floor.y);
        assert_eq!(3.0, floor.z);
    }

    #[test]
    fn floor_int() {
        let p = Point3::new(1, 2, 3);

        let floor = p.floor();

        assert_eq!(1, floor.x);
        assert_eq!(2, floor.y);
        assert_eq!(3, floor.z);
    }

    #[test]
    fn ceil() {
        let p = Point3::new(1.5, 2.7, 3.9);

        let ceil = p.ceil();

        assert_eq!(2.0, ceil.x);
        assert_eq!(3.0, ceil.y);
        assert_eq!(4.0, ceil.z);
    }

    #[test]
    fn ceil_int() {
        let p = Point3::new(1, 2, 3);

        let ceil = p.ceil();

        assert_eq!(1, ceil.x);
        assert_eq!(2, ceil.y);
        assert_eq!(3, ceil.z);
    }

    #[test]
    fn abs() {
        let p = Point3::new(-1.0, 2.0, -3.0);

        let abs = p.abs();

        assert_eq!(1.0, abs.x);
        assert_eq!(2.0, abs.y);
        assert_eq!(3.0, abs.z);
    }

    #[test]
    fn permute() {
        let p = Point3::new(1.0, 2.0, 3.0);

        let permuted = p.permute(2, 0, 1);

        assert_eq!(3.0, permuted.x);
        assert_eq!(1.0, permuted.y);
        assert_eq!(2.0, permuted.z);
    }

    #[test]
    fn from_vector3() {
        let v3 = Vector3{x: 1.0, y: 2.0, z: 3.0};
        let p3 = Point3::from(v3);

        assert_eq!(1.0, p3.x);
        assert_eq!(2.0, p3.y);
        assert_eq!(3.0, p3.z);
    }

    #[test]
    fn index() {
        let p = Point3::new(1.0, 2.0, 3.0);

        assert_eq!(1.0, p[0]);
        assert_eq!(2.0, p[1]);
        assert_eq!(3.0, p[2]);
    }

    #[test]
    fn index_mut() {
        let mut p = Point3::new(1.0, 2.0, 3.0);
        p[0] = 5.0;

        assert_eq!(5.0, p[0]);
        assert_eq!(2.0, p[1]);
        assert_eq!(3.0, p[2]);
    }

    #[test]
    fn add() {
        let p1 = Point3::new(1.0, 2.0, 3.0);
        let p2 = Point3::new(1.0, 2.0, 3.0);

        let result = p1 + p2;

        assert_eq!(2.0, result.x);
        assert_eq!(4.0, result.y);
        assert_eq!(6.0, result.z);
    }

    #[test]
    fn add_vector() {
        let p = Point3::new(1.0, 2.0, 3.0);
        let v = Vector3::new(1.0, 2.0, 3.0);

        let result = p + v;

        assert_eq!(2.0, result.x);
        assert_eq!(4.0, result.y);
        assert_eq!(6.0, result.z);
    }

    #[test]
    fn add_assign() {
        let mut p = Point3::new(1.0, 2.0, 3.0);
        p += Vector3::new(1.0, 2.0, 3.0);

        assert_eq!(2.0, p.x);
        assert_eq!(4.0, p.y);
        assert_eq!(6.0, p.z);
    }

    #[test]
    fn sub() {
        let p1 = Point3::new(1.0, 2.0, 3.0);
        let p2 = Point3::new(1.0, 2.0, 3.0);

        let result = p1 - p2;

        assert_eq!(0.0, result.x);
        assert_eq!(0.0, result.y);
        assert_eq!(0.0, result.z);
    }

    #[test]
    fn sub_vector() {
        let p = Point3::new(1.0, 2.0, 3.0);
        let v = Vector3::new(1.0, 2.0, 3.0);

        let result = p - v;

        assert_eq!(0.0, result.x);
        assert_eq!(0.0, result.y);
        assert_eq!(0.0, result.z);
    }

    #[test]
    fn sub_assign() {
        let mut p = Point3::new(1.0, 2.0, 3.0);
        p -= Vector3::new(1.0, 2.0, 3.0);

        assert_eq!(0.0, p.x);
        assert_eq!(0.0, p.y);
        assert_eq!(0.0, p.z);
    }

    #[test]
    fn mul() {
        let p1 = Point3::new(1.0, 2.0, 3.0);
        let p2 = p1 * 2.0;

        assert_eq!(2.0, p2.x);
        assert_eq!(4.0, p2.y);
        assert_eq!(6.0, p2.z);
    }

    #[test]
    fn mul_assign() {
        let mut p = Point3::new(1.0, 2.0, 3.0);
        p *= 2.0;

        assert_eq!(2.0, p.x);
        assert_eq!(4.0, p.y);
        assert_eq!(6.0, p.z);
    }
}
