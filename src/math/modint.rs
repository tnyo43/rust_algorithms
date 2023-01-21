use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Modint<const MOD: usize> {
    value: usize,
}

impl<const MOD: usize> Modint<MOD> {
    pub fn new(value: usize) -> Self {
        Self { value: value % MOD }
    }

    pub fn pow(self, index: usize) -> Self {
        let mut result = Self::new(1);
        let mut temp = Self::new(self.value);
        let mut n = index;

        while n > 0 {
            if n & 1 == 1 {
                result *= temp;
            }
            temp *= temp;
            n >>= 1;
        }

        result
    }

    pub fn inv(self) -> Self {
        self.pow(MOD - 2)
    }
}

impl<const MOD: usize> ops::Add for Modint<MOD> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new((self.value + rhs.value) % MOD)
    }
}

impl<const MOD: usize> ops::AddAssign for Modint<MOD> {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::new(self.value) + rhs;
    }
}
impl<const MOD: usize> ops::Sub for Modint<MOD> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new((MOD + self.value - rhs.value) % MOD)
    }
}

impl<const MOD: usize> ops::SubAssign for Modint<MOD> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self::new(self.value) - rhs
    }
}

impl<const MOD: usize> ops::Mul for Modint<MOD> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.value * rhs.value % MOD)
    }
}

impl<const MOD: usize> ops::MulAssign for Modint<MOD> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self::new(self.value) * rhs
    }
}

impl<const MOD: usize> ops::Div for Modint<MOD> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs.value == 0 {
            panic!("Zero Division Error.");
        }

        self * rhs.inv()
    }
}

impl<const MOD: usize> ops::DivAssign for Modint<MOD> {
    fn div_assign(&mut self, rhs: Self) {
        if rhs.value == 0 {
            panic!("Zero Division Error.");
        }

        *self = Self::new(self.value) / rhs.inv();
    }
}

#[cfg(test)]
mod tests {
    extern crate rstest;
    extern crate speculate;

    use rstest::*;
    use speculate::speculate;

    use super::*;

    const MOD: usize = 17;
    type M = Modint<MOD>;

    speculate! {
        describe "add in mod 17" {
            #[rstest(left, right, expected,
                case(0, 1, 1),
                case(4, 13, 0),
                case(4, 12, 16),
                case(10, 10, 3),
            )]
            fn test_add_mod_17(left: usize, right: usize, expected: usize) {
                assert_eq!(M::new(left) + M::new(right), M::new(expected));
            }
        }

        describe "subtract in mod 17" {
            #[rstest(left, right, expected,
                case(1, 0, 1),
                case(13, 14, 16),
                case(10, 123, 6),
            )]
            fn test_subtract_mod_17(left: usize, right: usize, expected: usize) {
                assert_eq!(M::new(left) - M::new(right), M::new(expected));
            }
        }

        describe "multiple in mod 17" {
            #[rstest(left, right, expected,
                case(1, 0, 0),
                case(3, 9, 10),
                case(16, 11, 6),
            )]
            fn test_multiple_mod_17(left: usize, right: usize, expected: usize) {
                assert_eq!(M::new(left) * M::new(right), M::new(expected));
            }
        }



        describe "power in mod 17" {
            #[rstest(base, index, expected,
                case(10, 0, 1),
                case(10, 1, 10),
                case(10, 2, 15),
                case(10, 3, 14),
                case(10, 4, 4),
                case(10, 5, 6),
                case(10, 16, 1),
            )]
            fn test_power_mod_17(base: usize, index: usize, expected: usize) {
                assert_eq!(M::new(base).pow(index), M::new(expected));
            }
        }

        describe "inverse in mod 17" {
            #[rstest(value, expected,
                case(10, 12),
                case(12, 10),
                case(2, 9),
                case(9, 2),
            )]
            fn test_inverse_mod_17(value: usize, expected: usize) {
                assert_eq!(M::new(value).inv(), M::new(expected));
                assert_eq!(M::new(value) * M::new(value).inv(), M::new(1));
            }
        }

        describe "divide in mod 17" {
            #[rstest(left, right, expected,
                case(16, 4, 4),
                case(10, 10, 1),
                case(3, 4, 5),
            )]
            fn test_divide_mod_17(left: usize, right: usize, expected: usize) {
                assert_eq!(M::new(left) / M::new(right), M::new(expected));
            }
        }
    }
}
