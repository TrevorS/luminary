use core::Point3f;
use core::Vector3f;

use core::medium::Medium;
use core::ray::Ray;

#[derive(Clone, Debug)]
pub struct RayDifferential {
    pub ray: Ray,
    pub rx_origin: Option<Point3f>,
    pub ry_origin: Option<Point3f>,
    pub rx_direction: Option<Vector3f>,
    pub ry_direction: Option<Vector3f>,
    pub has_differentials: bool,
}

impl RayDifferential {
    pub fn new(o: Point3f, d: Vector3f, medium: Medium, t_max: f64, time: f64) -> Self {
        Self::from(Ray::new(o, d, medium, t_max, time))
    }

    pub fn zero() -> Self {
        Self::from(Ray::zero())
    }

    pub fn at(self, t: f64) -> Point3f {
        self.ray.at(t)
    }

    pub fn scale_differentials(mut self, s: f64) {
        self.rx_origin = Some(self.ray.o + (self.rx_origin.unwrap() - self.ray.o) * s);
        self.rx_origin = Some(self.ray.o + (self.ry_origin.unwrap() - self.ray.o) * s);

        self.rx_direction = Some(self.ray.d + (self.rx_direction.unwrap() - self.ray.d) * s);
        self.ry_direction = Some(self.ray.d + (self.ry_direction.unwrap() - self.ray.d) * s);
    }
}

impl From<Ray> for RayDifferential {
    fn from(ray: Ray) -> Self {
        Self {
            ray: ray,
            rx_origin: None,
            ry_origin: None,
            rx_direction: None,
            ry_direction: None,
            has_differentials: false,
        }
    }
}
