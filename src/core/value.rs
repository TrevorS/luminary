use num::traits::{Signed, ToPrimitive, FromPrimitive};

pub trait Value: Signed + ToPrimitive + FromPrimitive + PartialOrd + Copy {
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
}

impl<T> Value for T where T: Signed + ToPrimitive + FromPrimitive + PartialOrd + Copy {}
