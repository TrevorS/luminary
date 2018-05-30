use core::utils::has_nans_2;
use core::value::Value;

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
}
