use std::ops::Index;

use core::point3::Point3;
use core::utils;
use core::transform::Transform;
use core::transformable::Transformable;
use core::value::Value;
use core::vector3::Vector3;

#[derive(Clone, Copy, Debug)]
pub struct Bounds3<T: Value> {
    p_min: Point3<T>,
    p_max: Point3<T>,
}

impl<T: Value> Bounds3<T> {
    pub fn new(p1: Point3<T>, p2: Point3<T>) -> Self {
        let p_min = Point3 {
            x: p1.x.min(p2.x),
            y: p1.y.min(p2.y),
            z: p1.z.min(p2.z),
        };

        let p_max = Point3 {
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
        let y_idx = if corner & 2 > 0 { 1 } else { 0 };
        let z_idx = if corner & 4 > 0 { 1 } else { 0 };

        Point3 {
            x: self[x_idx].x,
            y: self[y_idx].y,
            z: self[z_idx].z,
        }
    }

    pub fn union(self, p: Point3<T>) -> Self {
        Self {
            p_min: Point3 {
                x: self.p_min.x.min(p.x),
                y: self.p_min.y.min(p.y),
                z: self.p_min.z.min(p.z),
            },
            p_max: Point3 {
                x: self.p_max.x.min(p.x),
                y: self.p_max.y.min(p.y),
                z: self.p_max.z.min(p.z),
            },
        }
    }

    pub fn intersection(self, b: Bounds3<T>) -> Self {
        Self {
            p_min: Point3 {
                x: self.p_min.x.max(b.p_min.x),
                y: self.p_min.y.max(b.p_min.y),
                z: self.p_min.z.max(b.p_min.z),
            },
            p_max: Point3 {
                x: self.p_max.x.min(b.p_max.x),
                y: self.p_max.y.min(b.p_max.y),
                z: self.p_max.z.min(b.p_max.z),
            },
        }
    }

    pub fn overlap(self, b: Bounds3<T>) -> bool {
        let x = (self.p_max.x >= b.p_min.x) && (self.p_min.x <= b.p_max.x);
        let y = (self.p_max.y >= b.p_min.y) && (self.p_min.y <= b.p_max.y);
        let z = (self.p_max.z >= b.p_min.x) && (self.p_min.z <= b.p_max.z);

        x && y && z
    }

    pub fn inside(self, p: Point3<T>) -> bool {
        let x = (p.x >= self.p_min.x) && (p.x <= self.p_max.x);
        let y = (p.y >= self.p_min.y) && (p.y <= self.p_max.y);
        let z = (p.z >= self.p_min.z) && (p.z <= self.p_max.z);

        x && y && z
    }

    pub fn inside_exclusive(self, p: Point3<T>) -> bool {
        let x = (p.x >= self.p_min.x) && (p.x < self.p_max.x);
        let y = (p.y >= self.p_min.y) && (p.y < self.p_max.y);
        let z = (p.z >= self.p_min.z) && (p.z < self.p_max.z);

        x && y && z
    }

    pub fn expand(self, delta: T) -> Self {
        Self {
            p_min: self.p_min - Vector3 {
                x: delta,
                y: delta,
                z: delta,
            },
            p_max: self.p_max + Vector3 {
                x: delta,
                y: delta,
                z: delta,
            },
        }
    }

    pub fn diagonal(self) -> Vector3<T> {
        self.p_max - self.p_min
    }

    pub fn surface_area(self) -> T {
        let d = self.diagonal();

        (T::one() + T::one()) * (d.x * d.y + d.x * d.z + d.y * d.z)
    }

    pub fn maximum_extent(self) -> usize {
        let d = self.diagonal();

        if d.x > d.y && d.x > d.z {
            return 0;
        }

        if d.y > d.z {
            return 1;
        }

        return 2;
    }

    pub fn lerp(self, t: Point3<T>) -> Point3<T> {
        Point3 {
            x: utils::lerp(t.x, self.p_min.x, self.p_max.x),
            y: utils::lerp(t.y, self.p_min.y, self.p_max.y),
            z: utils::lerp(t.z, self.p_min.z, self.p_max.z),
        }
    }

    pub fn offset(self, p: Point3<T>) -> Vector3<T> {
        let mut o = p - self.p_min;

        if self.p_max.x > self.p_min.x {
            o.x = o.x / (self.p_max.x - self.p_min.x);
        }

        if self.p_max.y > self.p_min.y {
            o.y = o.x / (self.p_max.y - self.p_min.y);
        }

        if self.p_max.z > self.p_min.z {
            o.z = o.z / (self.p_max.z - self.p_min.z);
        }

        o
    }

    pub fn bounding_sphere(self) -> (Point3<T>, T) {
        let center = (self.p_min + self.p_max) / (T::one() + T::one());

        let mut radius = T::zero();

        if self.inside(center) {
            radius = self.p_max.distance(center)
        }

        (center, radius)
    }
}

impl<T: Value> Transformable for Bounds3<T> {
    fn transform(self, t: Transform) -> Self {
        let mut result = Bounds3::from(t.transform(Point3 {
            x: self.p_min.x,
            y: self.p_min.y,
            z: self.p_min.z,
        }));

        result = result.union(t.transform(Point3 {
            x: self.p_max.x,
            y: self.p_min.y,
            z: self.p_min.z,
        }));

        result = result.union(t.transform(Point3 {
            x: self.p_min.x,
            y: self.p_max.y,
            z: self.p_min.z,
        }));

        result = result.union(t.transform(Point3 {
            x: self.p_min.x,
            y: self.p_min.y,
            z: self.p_max.z,
        }));

        result = result.union(t.transform(Point3 {
            x: self.p_min.x,
            y: self.p_max.y,
            z: self.p_max.z,
        }));

        result = result.union(t.transform(Point3 {
            x: self.p_max.x,
            y: self.p_max.y,
            z: self.p_min.z,
        }));

        result = result.union(t.transform(Point3 {
            x: self.p_max.x,
            y: self.p_min.y,
            z: self.p_max.z,
        }));

        result = result.union(t.transform(Point3 {
            x: self.p_max.x,
            y: self.p_max.y,
            z: self.p_max.z,
        }));

        result
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
