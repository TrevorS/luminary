use std::f64;

use core::Point3f;
use core::Vector3f;
use core::transform::Transform;
use core::transformable::Transformable;

use core::medium::Medium;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub o: Point3f,
    pub d: Vector3f,
    pub medium: Medium,
    pub t_max: f64,
    pub time: f64,
}

impl Ray {
    pub fn new(o: Point3f, d: Vector3f, medium: Medium, t_max: f64, time: f64) -> Self {
        Self {
            o,
            d,
            medium,
            t_max,
            time,
        }
    }

    pub fn zero() -> Self {
        Self::new(
            Point3f::zero(),
            Vector3f::zero(),
            Medium {},
            f64::INFINITY,
            0.0,
        )
    }

    pub fn at(self, t: f64) -> Point3f {
        // TODO: Look into switching to nightly in order to use () operators.
        self.o + self.d * t
    }
}

impl Transformable for Ray {
    fn transform(self, t: Transform) -> Self {
        let o_error = Vector3f::zero();

        let mut o = t.transform(self.o);
        let d = t.transform(self.d);

        let length_squared = d.length_squared();
        let mut t_max = self.t_max;

        if length_squared > 0.0 {
            let dt = d.abs().dot(o_error) / length_squared;
            o += d * dt;
            t_max -= dt;
        }

        Ray { o, d, t_max, time: self.time, medium: self.medium }
    }
}
