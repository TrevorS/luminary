use std::ops::{
    Add,
    AddAssign,
    Sub,
    SubAssign,
};

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

        Point3{ x: x, y: y, z: z }
    }

    pub fn distance_squared(self, other: Self) -> T {
        (self - other).length_squared()
    }

    pub fn distance(self, other: Self) -> T {
        (self - other).length()
    }
}

impl<T: Value> From<Vector3<T>> for Point3<T> {
    fn from(v: Vector3<T>) -> Self {
        Point3{
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl<T: Value> Add<Vector3<T>> for Point3<T> {
    type Output = Self;

    fn add(self, other: Vector3<T>) -> Self {
        Point3{
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
        Point3{
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
    fn from_vector3() {
        let v3 = Vector3{x: 1.0, y: 2.0, z: 3.0};
        let p3 = Point3::from(v3);

        assert_eq!(1.0, p3.x);
        assert_eq!(2.0, p3.y);
        assert_eq!(3.0, p3.z);
    }

    #[test]
    fn add() {
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
    fn sub_point() {
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
}
