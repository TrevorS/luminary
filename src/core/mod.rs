pub mod bounds2;
pub mod bounds3;
pub mod matrix44;
pub mod medium;
pub mod normal3;
pub mod point2;
pub mod point3;
pub mod ray;
pub mod ray_differential;
pub mod transform;
pub mod transformable;
pub mod utils;
pub mod value;
pub mod vector2;
pub mod vector3;

pub type Normal3f = normal3::Normal3<f64>;

pub type Bounds2f = bounds2::Bounds2<f64>;
pub type Bounds2i = bounds2::Bounds2<i32>;
pub type Bounds3f = bounds3::Bounds3<f64>;
pub type Bounds3i = bounds3::Bounds3<i32>;

pub type Point2f = point2::Point2<f64>;
pub type Point2i = point2::Point2<i32>;
pub type Point3f = point3::Point3<f64>;
pub type Point3i = point3::Point3<i32>;

pub type Vector2f = vector2::Vector2<f64>;
pub type Vector2i = vector2::Vector2<i32>;
pub type Vector3f = vector3::Vector3<f64>;
pub type Vector3i = vector3::Vector3<i32>;
