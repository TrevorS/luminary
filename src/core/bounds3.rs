use std::ops::Index;

use core::point3::Point3;
use core::value::Value;

#[derive(Clone, Debug)]
pub struct Bounds3<T: Value> {
    p_min: Point3<T>,
    p_max: Point3<T>,
}

impl<T: Value> Bounds3<T> {
    pub fn new(p1: Point3<T>, p2: Point3<T>) -> Self {
        let p_min = Point3{
            x: p1.x.min(p2.x),
            y: p1.y.min(p2.y),
            z: p1.z.min(p2.z),
        };

        let p_max = Point3{
            x: p1.x.max(p2.x),
            y: p1.y.max(p2.y),
            z: p1.z.max(p2.z),
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

    pub fn corner(self, corner: usize) -> Point3<T> {
        let x_idx = corner & 1;

        let y_idx = if corner & 2 > 0 {
            1
        } else {
            0
        };

        let z_idx = if corner & 4 > 0 {
            1
        } else {
            0
        };

        Point3 {
            x: self[x_idx].x,
            y: self[y_idx].y,
            z: self[z_idx].z,
        }
    }
}

impl<T: Value> From<Point3<T>> for Bounds3<T> {
    fn from(p: Point3<T>) -> Self {
        Self::new(p, p)
    }
}

impl<T: Value> Index<usize> for Bounds3<T> {
    type Output = Point3<T>;

    fn index(&self, i: usize) -> &Point3<T> {
        assert!(i <= 1);

        match i {
            0 => &self.p_min,
            _ => &self.p_max,
        }
    }
}
