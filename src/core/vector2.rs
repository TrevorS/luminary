use std::ops::{
    Add,
    AddAssign,
    Div,
    DivAssign,
    Index,
    IndexMut,
    Mul,
    MulAssign,
    Neg,
    Sub,
    SubAssign,
};

use core::value::Value;

#[derive(Clone, Copy, Debug)]
pub struct Vector2<T: Value> {
    pub x: T,
    pub y: T,
}

impl<T: Value> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        let v = Vector2 { x: x, y: y };

        assert!(!v.has_nans());

        v
    }

    fn has_nans(self) -> bool {
        self.x != self.x || self.y != self.y
    }

    pub fn abs(self) -> Self {
        Vector2{
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y
    }

    pub fn abs_dot(self, other: Self) -> T {
        self.dot(other).abs()
    }

    pub fn length_squared(self) -> T {
        self.x * self.x + self.y * self.y
    }

    pub fn length(self) -> T {
        self.length_squared().sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.length()
    }

    pub fn min_component(self) -> T {
        self.x.min(self.y)
    }

    pub fn max_component(self) -> T {
        self.x.max(self.y)
    }

    pub fn max_dimension(self) -> usize {
        if self.x > self.y {
            0
        } else {
            1
        }
    }

    pub fn min(self, other: Self) -> Self {
        Vector2{
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    pub fn max(self, other: Self) -> Self {
        Vector2{
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    pub fn permute(self, x: usize, y: usize) -> Self {
        Vector2{
            x: self[x],
            y: self[y],
        }
    }
}

impl<T: Value> Index<usize> for Vector2<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        assert!(i <= 1);

        match i {
            0 => &self.x,
            _ => &self.y,
        }
    }
}

impl<T: Value> IndexMut<usize> for Vector2<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        assert!(i <= 1);

        match i {
            0 => &mut self.x,
            _ => &mut self.y,
        }
    }
}

impl<T: Value> Add for Vector2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector2{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Value + AddAssign> AddAssign for Vector2<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: Value> Sub for Vector2<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector2{
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Value + SubAssign> SubAssign for Vector2<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<T: Value> Mul<T> for Vector2<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Vector2{
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl<T: Value + MulAssign> MulAssign<T> for Vector2<T> {
    fn mul_assign(&mut self, other: T) {
        self.x *= other;
        self.y *= other;
    }
}

impl<T: Value> Div<T> for Vector2<T> {
    type Output = Self;

    fn div(self, other: T) -> Self {
        let inv = T::one() / other;

        Vector2{
            x: self.x * inv,
            y: self.y * inv,
        }
    }
}

impl<T: Value + MulAssign> DivAssign<T> for Vector2<T> {
    fn div_assign(&mut self, other: T) {
        let inv = T::one() / other;

        self.x *= inv;
        self.y *= inv;
    }
}

impl<T: Value> Neg for Vector2<T> {
    type Output = Self;

    fn neg(self) -> Self {
        let neg_one = T::one().neg();

        Vector2{
            x: neg_one * self.x,
            y: neg_one * self.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64;

    #[test]
    fn new() {
        let v = Vector2::new(1.0, 2.0);

        assert_eq!(1.0, v.x);
        assert_eq!(2.0, v.y);
    }

    #[test]
    fn new_int() {
        let v = Vector2::new(1, 2);

        assert_eq!(1, v.x);
        assert_eq!(2, v.y);
    }

    #[test]
    #[should_panic]
    fn has_nans_true() {
        Vector2::new(1.0, f64::NAN);
    }

    #[test]
    fn has_nans_false() {
        let v = Vector2::new(1.0, 2.0);

        assert_eq!(false, v.has_nans())
    }

    #[test]
    fn abs() {
        let v1 = Vector2::new(-1.0, -2.0);
        let v2 = v1.abs();

        assert_eq!(1.0, v2.x);
        assert_eq!(2.0, v2.y);
    }

    #[test]
    fn dot() {
        let v1 = Vector2::new(1.0, 2.0);
        let v2 = Vector2::new(1.0, 2.0);

        let dot = v1.dot(v2);

        assert_eq!(5.0, dot);
    }

    #[test]
    fn abs_dot() {
        let v1 = Vector2::new(1.0, 2.0);
        let v2 = Vector2::new(-1.0, -2.0);

        let abs_dot = v1.abs_dot(v2);

        assert_eq!(5.0, abs_dot);
    }

    #[test]
    fn length_squared() {
        let v = Vector2::new(1.0, 2.0);

        let length_squared = v.length_squared();

        assert_eq!(5.0, length_squared);
    }

    #[test]
    fn length() {
        let v = Vector2::new(1.0, 2.0);

        let length = v.length();

        assert_eq!(2.23606797749979, length);
    }

    #[test]
    fn normalize() {
        let v = Vector2::new(3.0, 1.0);

        let normalized = v.normalize();

        assert_eq!(0.9486832980505138, normalized.x);
        assert_eq!(0.31622776601683794, normalized.y);
    }

    #[test]
    fn min_component() {
        let v = Vector2::new(1.0, 2.0);

        let min_component = v.min_component();

        assert_eq!(1.0, min_component);
    }

    #[test]
    fn max_component() {
        let v = Vector2::new(1.0, 2.0);

        let max_component = v.max_component();

        assert_eq!(2.0, max_component);
    }

    #[test]
    fn max_dimension() {
        let v = Vector2::new(1.0, 2.0);

        let max_dimension = v.max_dimension();

        assert_eq!(1, max_dimension);
    }

    #[test]
    fn min() {
        let v1 = Vector2::new(1.0, 2.0);
        let v2 = Vector2::new(3.0, 2.0);

        let min = v1.min(v2);

        assert_eq!(1.0, min.x);
        assert_eq!(2.0, min.y);
    }

    #[test]
    fn max() {
        let v1 = Vector2::new(1.0, 2.0);
        let v2 = Vector2::new(3.0, 2.0);

        let min = v1.max(v2);

        assert_eq!(3.0, min.x);
        assert_eq!(2.0, min.y);
    }

    #[test]
    fn permute() {
        let v = Vector2::new(1.0, 2.0);

        let permuted = v.permute(1, 0);

        assert_eq!(2.0, permuted.x);
        assert_eq!(1.0, permuted.y);
    }

    #[test]
    fn index() {
        let v = Vector2::new(1.0, 2.0);

        assert_eq!(1.0, v[0]);
        assert_eq!(2.0, v[1]);
    }

    #[test]
    fn index_mut() {
        let mut v = Vector2::new(1.0, 2.0);
        v[0] = 5.0;

        assert_eq!(5.0, v[0]);
        assert_eq!(2.0, v[1]);
    }

    #[test]
    fn plus() {
        let v1 = Vector2::new(1.0, 2.0);
        let v2 = Vector2::new(1.0, 2.0);

        let v3 = v1 + v2;

        assert_eq!(2.0, v3.x);
        assert_eq!(4.0, v3.y);
    }

    #[test]
    fn plus_assign() {
        let mut v1 = Vector2::new(1.0, 2.0);
        v1 += Vector2::new(1.0, 2.0);

        assert_eq!(2.0, v1.x);
        assert_eq!(4.0, v1.y);
    }

    #[test]
    fn sub() {
        let v1 = Vector2::new(1.0, 2.0);
        let v2 = Vector2::new(1.0, 2.0);

        let v3 = v1 - v2;

        assert_eq!(0.0, v3.x);
        assert_eq!(0.0, v3.y);
    }

    #[test]
    fn sub_assign() {
        let mut v1 = Vector2::new(1.0, 2.0);
        v1 -= Vector2::new(1.0, 2.0);

        assert_eq!(0.0, v1.x);
        assert_eq!(0.0, v1.y);
    }

    #[test]
    fn mul() {
        let v1 = Vector2::new(1.0, 2.0);
        let v2 = v1 * 2.0;

        assert_eq!(2.0, v2.x);
        assert_eq!(4.0, v2.y);
    }

    #[test]
    fn mul_assign() {
        let mut v1 = Vector2::new(1.0, 2.0);
        v1 *= 2.0;

        assert_eq!(2.0, v1.x);
        assert_eq!(4.0, v1.y);
    }

    #[test]
    fn div() {
        let v1 = Vector2::new(2.0, 4.0);
        let v2 = v1 / 2.0;

        assert_eq!(1.0, v2.x);
        assert_eq!(2.0, v2.y);
    }

    #[test]
    fn div_assign() {
        let mut v1 = Vector2::new(2.0, 4.0);
        v1 /= 2.0;

        assert_eq!(1.0, v1.x);
        assert_eq!(2.0, v1.y);
    }

    #[test]
    fn neg() {
        let v1 = -Vector2::new(2.0, 4.0);

        assert_eq!(-2.0, v1.x);
        assert_eq!(-4.0, v1.y);
    }
}
