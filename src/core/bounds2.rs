use std::ops::Index;

use core::point2::Point2;
use core::value::Value;

#[derive(Clone, Debug)]
pub struct Bounds2<T: Value> {
    p_min: Point2<T>,
    p_max: Point2<T>,
}

impl<T: Value> Bounds2<T> {
    pub fn new(p1: Point2<T>, p2: Point2<T>) -> Self {
        let p_min = Point2{
            x: p1.x.min(p2.x),
            y: p1.y.min(p2.y),
        };

        let p_max = Point2{
            x: p1.x.max(p2.x),
            y: p1.y.max(p2.y),
        };

        Self {
            p_min: p_min,
            p_max: p_max,
        }
    }

    pub fn zero() -> Self {
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
        Self::new(p, p)
    }
}

impl<T: Value> Index<usize> for Bounds2<T> {
    type Output = Point2<T>;

    fn index(&self, i: usize) -> &Point2<T> {
        assert!(i <= 1);

        match i {
            0 => &self.p_min,
            _ => &self.p_max,
        }
    }
}
