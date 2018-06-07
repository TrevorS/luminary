use core::point3::Point3;
use core::value::Value;

#[derive(Clone, Debug)]
pub struct Bounds3<T: Value> {
    p_min: Point3<T>,
    p_max: Point3<T>,
}

impl<T: Value> Bounds3<T> {
    pub fn new() -> Self {
        let min_num = T::min_value();
        let max_num = T::max_value();

        Self {
            p_min: Point3 {
                x: max_num,
                y: max_num,
                z: max_num,
            },
            p_max: Point3 {
                x: min_num,
                y: min_num,
                z: min_num,
            },
        }
    }
}

impl<T: Value> From<Point3<T>> for Bounds3<T> {
    fn from(p: Point3<T>) -> Self {
        Self { p_min: p, p_max: p }
    }
}
