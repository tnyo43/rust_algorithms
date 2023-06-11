use std::{fmt::Display, ops};

use num_traits::{Num, ToPrimitive};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Point2<Data: Num>(pub Data, pub Data);

impl<Data> Display for Point2<Data>
where
    Data: Num + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl<Data: Num> Point2<Data> {
    pub fn new(x: Data, y: Data) -> Self {
        Self(x, y)
    }
}

impl<Data: Num> ops::Add for Point2<Data> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<Data: Num + Copy> ops::AddAssign for Point2<Data> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl<Data: Num> ops::Sub for Point2<Data> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<Data: Num + Copy> ops::SubAssign for Point2<Data> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl<Data: Num + Copy> ops::Mul<Data> for Point2<Data> {
    type Output = Self;

    fn mul(self, rhs: Data) -> Self::Output {
        Self::new(self.0 * rhs, self.1 * rhs)
    }
}

impl<Data: Num + Copy> ops::MulAssign<Data> for Point2<Data> {
    fn mul_assign(&mut self, rhs: Data) {
        *self = *self * rhs
    }
}

impl<Data: Num + ToPrimitive> Point2<Data> {
    pub fn distance(self, rhs: Self) -> f64 {
        ((self.0 - rhs.0).to_f64().unwrap().powf(2.0)
            + (self.1 - rhs.1).to_f64().unwrap().powf(2.0))
        .sqrt()
    }

    pub fn arg(self) -> f64 {
        self.1.to_f64().unwrap().atan2(self.0.to_f64().unwrap())
    }
}

impl<Data: Num> Point2<Data> {
    pub fn det(self, rhs: Self) -> Data {
        self.0 * rhs.1 - self.1 * rhs.0
    }
}
