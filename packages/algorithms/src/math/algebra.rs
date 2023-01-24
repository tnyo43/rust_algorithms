pub trait Semigroup {
    fn op(&self, rhs: Self) -> Self;
}

pub trait Monoid: Semigroup {
    fn id() -> Self;
}

pub trait ActOpMonoid<T>: Monoid
where
    T: Monoid,
{
    fn act(&self, rhs: T) -> Self;
}
