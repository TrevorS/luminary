use num::traits::{Bounded, FromPrimitive, NumCast, Signed, ToPrimitive};

pub trait Value:
    Bounded + Signed + NumCast + ToPrimitive + FromPrimitive + PartialOrd + PartialEq + Copy
{
    fn min(self, other: Self) -> Self {
        if self < other {
            self
        } else {
            other
        }
    }

    fn max(self, other: Self) -> Self {
        if self > other {
            self
        } else {
            other
        }
    }

    fn sqrt(self) -> Self {
        let result = self.to_f64().unwrap().sqrt();

        FromPrimitive::from_f64(result).unwrap()
    }

    fn floor(self) -> Self {
        let result = self.to_f64().unwrap().floor();

        FromPrimitive::from_f64(result).unwrap()
    }

    fn ceil(self) -> Self {
        let result = self.to_f64().unwrap().ceil();

        FromPrimitive::from_f64(result).unwrap()
    }
}

impl<T> Value for T
where
    T: Bounded + Signed + NumCast + ToPrimitive + FromPrimitive + PartialOrd + PartialEq + Copy,
{
}
