pub trait Semigroup {
    fn op(&self, rhs: Self) -> Self;
}

pub trait Monoid: Semigroup {
    fn id() -> Self;
}
