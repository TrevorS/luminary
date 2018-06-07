use core::point2::Point2;
use core::value::Value;

#[derive(Clone, Debug)]
pub struct Bounds2<T: Value> {
    p_min: Point2<T>,
    p_max: Point2<T>,
}

impl<T: Value> Bounds2<T> {
    pub fn new() -> Self {
        let min_num = T::min_value();
        let max_num = T::max_value();

        Self {
            p_min: Point2 {
                x: max_num,
                y: max_num,
            },
            p_max: Point2 {
                x: min_num,
                y: min_num,
            },
        }
    }
}

impl<T: Value> From<Point2<T>> for Bounds2<T> {
    fn from(p: Point2<T>) -> Self {
        Self { p_min: p, p_max: p }
    }
}
