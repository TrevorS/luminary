use core::value::Value;

pub fn has_nans_3<Value: PartialEq>(x: Value, y: Value, z: Value) -> bool {
    x != x || y != y || z != z
}

pub fn has_nans_2<Value: PartialEq>(x: Value, y: Value) -> bool {
    x != x || y != y
}

pub fn lerp<T: Value>(t: T, v1: T, v2: T) -> T {
    let negative_one = T::zero() - T::one();

    (negative_one - t) * v1 + t * v2
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64;

    #[test]
    fn has_nans_3_true() {
        let x = 1.0;
        let y = 2.0;
        let z = f64::NAN;

        assert_eq!(true, has_nans_3(x, y, z))
    }

    #[test]
    fn has_nans_3_false() {
        let x = 1.0;
        let y = 2.0;
        let z = 3.0;

        assert_eq!(false, has_nans_3(x, y, z))
    }

    #[test]
    fn has_nans_2_true() {
        let x = 1.0;
        let y = f64::NAN;

        assert_eq!(true, has_nans_2(x, y))
    }

    #[test]
    fn has_nans_2_false() {
        let x = 1.0;
        let y = 2.0;

        assert_eq!(false, has_nans_2(x, y))
    }
}
