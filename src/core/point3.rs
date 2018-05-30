use core::utils::has_nans_3;
use core::value::Value;

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64;

    #[test]
    fn new() {
        let v = Point3::new(1.0, 2.0, 3.0);

        assert_eq!(1.0, v.x);
        assert_eq!(2.0, v.y);
        assert_eq!(3.0, v.z);
    }

    #[test]
    fn new_int() {
        let v = Point3::new(1, 2, 3);

        assert_eq!(1, v.x);
        assert_eq!(2, v.y);
        assert_eq!(3, v.z);
    }

    #[test]
    #[should_panic]
    fn has_nans_true() {
        Point3::new(1.0, 2.0, f64::NAN);
    }
}
