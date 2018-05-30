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

    #[test]
    fn from_vector3() {
        let v3 = Vector3{x: 1.0, y: 2.0, z: 3.0};
        let p3 = Point3::from(v3);

        assert_eq!(1.0, p3.x);
        assert_eq!(2.0, p3.y);
        assert_eq!(3.0, p3.z);
    }
}
