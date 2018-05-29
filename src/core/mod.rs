pub mod vector2;
pub mod vector3;

use num::traits::{FromPrimitive, ToPrimitive, Signed};

pub type Vector2f = vector2::Vector2<f64>;
pub type Vector2i = vector2::Vector2<i32>;

pub type Vector3f = vector3::Vector3<f64>;
pub type Vector3i = vector3::Vector3<i32>;

pub fn min<T: Signed + PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

pub fn max<T: Signed + PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

pub fn sqrt<T: Signed + ToPrimitive + FromPrimitive + PartialOrd>(value: T) -> T {
    let result = value.to_f64().unwrap().sqrt();

    FromPrimitive::from_f64(result).unwrap()
}
