use std::ops;

#[derive(Clone, Copy)]
pub struct Point2<Data>(Data, Data);

impl<Data> Point2<Data> {
    pub fn new(x: Data, y: Data) -> Self {
        Self(x, y)
    }
}

impl<Data> ops::Add for Point2<Data>
where
    Data: ops::Add<Output = Data>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<Data> ops::AddAssign for Point2<Data>
where
    Data: ops::Add<Output = Data> + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl<Data> ops::Sub for Point2<Data>
where
    Data: ops::Sub<Output = Data>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<Data> ops::SubAssign for Point2<Data>
where
    Data: ops::Sub<Output = Data> + Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl<Data> ops::Mul<Data> for Point2<Data>
where
    Data: ops::Mul<Output = Data> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Data) -> Self::Output {
        Self::new(self.0 * rhs, self.1 * rhs)
    }
}

impl<Data> ops::MulAssign<Data> for Point2<Data>
where
    Data: ops::Mul<Output = Data> + Copy,
{
    fn mul_assign(&mut self, rhs: Data) {
        *self = *self * rhs
    }
}

impl<Data> Point2<Data>
where
    Data: ops::Sub<Output = Data> + ops::Mul<Output = Data>,
{
    pub fn det(self, rhs: Self) -> Data {
        self.0 * rhs.1 - self.1 * rhs.0
    }
}
