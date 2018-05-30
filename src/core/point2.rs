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

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64;

    #[test]
    fn new() {
        let v = Point2::new(1.0, 2.0);

        assert_eq!(1.0, v.x);
        assert_eq!(2.0, v.y);
    }

    #[test]
    fn new_int() {
        let v = Point2::new(1, 2);

        assert_eq!(1, v.x);
        assert_eq!(2, v.y);
    }

    #[test]
    #[should_panic]
    fn has_nans_true() {
        Point2::new(1.0, f64::NAN);
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
}
