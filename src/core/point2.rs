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

use core::utils::has_nans_2;
use core::point3::Point3;
use core::value::Value;
use core::vector2::Vector2;

#[derive(Clone, Copy, Debug)]
pub struct Point2<T: Value> {
    pub x: T,
    pub y: T,
}

impl<T: Value> Point2<T> {
    pub fn new(x: T, y: T) -> Self {
        assert!(!has_nans_2(x, y));

        Point2{ x: x, y: y }
    }

    pub fn min(self, other: Self) -> Self {
        Point2{
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    pub fn max(self, other: Self) -> Self {
        Point2{
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    pub fn floor(self) -> Self {
        Point2{
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }

    pub fn ceil(self) -> Self {
        Point2{
            x: self.x.ceil(),
            y: self.y.ceil(),
        }
    }

    pub fn abs(self) -> Self {
        Point2{
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    pub fn permute(self, x: usize, y: usize) -> Self {
        Point2{
            x: self[x],
            y: self[y],
        }
    }
}

impl<T: Value> From<Point3<T>> for Point2<T> {
    fn from(p: Point3<T>) -> Self {
        Point2{
            x: p.x,
            y: p.y,
        }
    }
}

impl<T: Value> From<Vector2<T>> for Point2<T> {
    fn from(v: Vector2<T>) -> Self {
        Point2{
            x: v.x,
            y: v.y,
        }
    }
}

impl<T: Value> Index<usize> for Point2<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        assert!(i <= 1);

        match i {
            0 => &self.x,
            _ => &self.y,
        }
    }
}

impl<T: Value> IndexMut<usize> for Point2<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        assert!(i <= 1);

        match i {
            0 => &mut self.x,
            _ => &mut self.y,
        }
    }
}

impl<T: Value> Add for Point2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point2{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Value> Add<Vector2<T>> for Point2<T> {
    type Output = Self;

    fn add(self, other: Vector2<T>) -> Self {
        Point2{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Value + AddAssign> AddAssign<Vector2<T>> for Point2<T> {
    fn add_assign(&mut self, other: Vector2<T>) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: Value> Sub<Self> for Point2<T> {
    type Output = Vector2<T>;

    fn sub(self, other: Self) -> Vector2<T> {
        Vector2{
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Value> Sub<Vector2<T>> for Point2<T> {
    type Output = Self;

    fn sub(self, other: Vector2<T>) -> Self {
        Point2{
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Value + SubAssign> SubAssign<Vector2<T>> for Point2<T> {
    fn sub_assign(&mut self, other: Vector2<T>) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<T: Value> Mul<T> for Point2<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Point2{
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl<T: Value + MulAssign> MulAssign<T> for Point2<T> {
    fn mul_assign(&mut self, other: T) {
        self.x *= other;
        self.y *= other;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64;

    #[test]
    fn new() {
        let p = Point2::new(1.0, 2.0);

        assert_eq!(1.0, p.x);
        assert_eq!(2.0, p.y);
    }

    #[test]
    fn new_int() {
        let p = Point2::new(1, 2);

        assert_eq!(1, p.x);
        assert_eq!(2, p.y);
    }

    #[test]
    #[should_panic]
    fn has_nans_true() {
        Point3::new(1.0, 2.0, f64::NAN);
    }

    #[test]
    fn min() {
        let p1 = Point2::new(1.0, 2.0);
        let p2 = Point2::new(3.0, 2.0);

        let min = p1.min(p2);

        assert_eq!(1.0, min.x);
        assert_eq!(2.0, min.y);
    }

    #[test]
    fn max() {
        let p1 = Point2::new(1.0, 2.0);
        let p2 = Point2::new(3.0, 2.0);

        let min = p1.max(p2);

        assert_eq!(3.0, min.x);
        assert_eq!(2.0, min.y);
    }

    #[test]
    fn floor() {
        let p = Point2::new(1.1, 2.2);

        let floor = p.floor();

        assert_eq!(1.0, floor.x);
        assert_eq!(2.0, floor.y);
    }

    #[test]
    fn floor_int() {
        let p = Point2::new(1, 2);

        let floor = p.floor();

        assert_eq!(1, floor.x);
        assert_eq!(2, floor.y);
    }

    #[test]
    fn ceil() {
        let p = Point2::new(1.5, 2.7);

        let ceil = p.ceil();

        assert_eq!(2.0, ceil.x);
        assert_eq!(3.0, ceil.y);
    }

    #[test]
    fn ceil_int() {
        let p = Point2::new(1, 2);

        let ceil = p.ceil();

        assert_eq!(1, ceil.x);
        assert_eq!(2, ceil.y);
    }

    #[test]
    fn abs() {
        let p = Point2::new(-1.0, 2.0);

        let abs = p.abs();

        assert_eq!(1.0, abs.x);
        assert_eq!(2.0, abs.y);
    }

    #[test]
    fn permute() {
        let p = Point2::new(1.0, 2.0);

        let permuted = p.permute(1, 0);

        assert_eq!(2.0, permuted.x);
        assert_eq!(1.0, permuted.y);
    }

    #[test]
    fn from_point3() {
        let p3 = Point3{x: 1.0, y: 2.0, z: 3.0};
        let p2 = Point2::from(p3);

        assert_eq!(1.0, p2.x);
        assert_eq!(2.0, p2.y);
    }

    #[test]
    fn from_vector2() {
        let v2 = Vector2{x: 1.0, y: 2.0};
        let p2 = Point2::from(v2);

        assert_eq!(1.0, p2.x);
        assert_eq!(2.0, p2.y);
    }

    #[test]
    fn index() {
        let p = Point2::new(1.0, 2.0);

        assert_eq!(1.0, p[0]);
        assert_eq!(2.0, p[1]);
    }

    #[test]
    fn index_mut() {
        let mut p = Point2::new(1.0, 2.0);
        p[0] = 5.0;

        assert_eq!(5.0, p[0]);
        assert_eq!(2.0, p[1]);
    }

    #[test]
    fn add() {
        let p1 = Point2::new(1.0, 2.0);
        let p2 = Point2::new(1.0, 2.0);

        let result = p1 + p2;

        assert_eq!(2.0, result.x);
        assert_eq!(4.0, result.y);
    }

    #[test]
    fn add_vector() {
        let p = Point2::new(1.0, 2.0);
        let v = Vector2::new(1.0, 2.0);

        let result = p + v;

        assert_eq!(2.0, result.x);
        assert_eq!(4.0, result.y);
    }

    #[test]
    fn add_assign() {
        let mut p = Point2::new(1.0, 2.0);
        p += Vector2::new(1.0, 2.0);

        assert_eq!(2.0, p.x);
        assert_eq!(4.0, p.y);
    }

    #[test]
    fn sub() {
        let p1 = Point2::new(1.0, 2.0);
        let p2 = Point2::new(1.0, 2.0);

        let result = p1 - p2;

        assert_eq!(0.0, result.x);
        assert_eq!(0.0, result.y);
    }

    #[test]
    fn sub_vector() {
        let p = Point2::new(1.0, 2.0);
        let v = Vector2::new(1.0, 2.0);

        let result = p - v;

        assert_eq!(0.0, result.x);
        assert_eq!(0.0, result.y);
    }

    #[test]
    fn sub_assign() {
        let mut p = Point2::new(1.0, 2.0);
        p -= Vector2::new(1.0, 2.0);

        assert_eq!(0.0, p.x);
        assert_eq!(0.0, p.y);
    }

    #[test]
    fn mul() {
        let p1 = Point2::new(1.0, 2.0);
        let p2 = p1 * 2.0;

        assert_eq!(2.0, p2.x);
        assert_eq!(4.0, p2.y);
    }

    #[test]
    fn mul_assign() {
        let mut p = Point2::new(1.0, 2.0);
        p *= 2.0;

        assert_eq!(2.0, p.x);
        assert_eq!(4.0, p.y);
    }
}
