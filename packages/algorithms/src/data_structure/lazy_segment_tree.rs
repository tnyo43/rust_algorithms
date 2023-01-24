use std::fmt::Display;

use crate::math::algebra::{ActOpMonoid, Monoid};

pub trait SegmentPropertation {
    fn p(&self, len: u32) -> Self;
}

pub struct LazySegmentTree<M, A>
where
    M: ActOpMonoid<A> + Clone + Copy + Display,
    A: Monoid + SegmentPropertation + Clone + Copy + PartialEq + Display,
{
    size: usize,
    data: Vec<M>,
    lazy: Vec<A>,
}

fn print<T: Display>(
    f: &mut std::fmt::Formatter<'_>,
    data: &Vec<T>,
    size: usize,
) -> std::fmt::Result {
    for (i, v) in data.iter().enumerate() {
        let n = 1 << ((i + 1).leading_zeros() - size.leading_zeros());

        for _ in 0..(n / 2) {
            write!(f, "\t")?;
        }
        write!(f, "{}", v)?;
        for _ in 0..((n + 1) / 2) {
            write!(f, "\t")?;
        }

        if (i + 2).count_ones() == 1 {
            write!(f, "\n")?;
        }
    }
    write!(f, "")
}

impl<M, A> Display for LazySegmentTree<M, A>
where
    M: ActOpMonoid<A> + Clone + Copy + Display,
    A: Monoid + SegmentPropertation + Clone + Copy + PartialEq + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[[data]]\n")?;
        print(f, &self.data, self.size)?;
        write!(f, "\n[[lazy]]\n")?;
        print(f, &self.lazy, self.size)?;
        write!(f, "")
    }
}

impl<M, A> LazySegmentTree<M, A>
where
    M: ActOpMonoid<A> + Clone + Copy + Display,
    A: Monoid + SegmentPropertation + Clone + Copy + PartialEq + Display,
{
    pub fn new(size: usize) -> Self {
        let mut s = 1;
        while s < size {
            s *= 2;
        }

        LazySegmentTree::<M, A> {
            size: s,
            data: vec![M::id(); 2 * s - 1],
            lazy: vec![A::id(); 2 * s - 1],
        }
    }

    pub fn from(data: &Vec<M>) -> Self {
        let mut s = 1;
        while s < data.len() {
            s *= 2;
        }

        let mut d = vec![M::id(); 2 * s - 1];

        for (i, x) in data.iter().enumerate() {
            d[s - 1 + i] = x.clone();
        }
        for i in (0..s - 1).rev() {
            d[i] = d[i * 2 + 1].op(d[i * 2 + 2]);
        }

        LazySegmentTree::<M, A> {
            size: s,
            data: d,
            lazy: vec![A::id(); 2 * s - 1],
        }
    }

    fn is_bottom(&self, k: usize) -> bool {
        k >= self.size - 1
    }

    pub fn eval(&mut self, k: usize) {
        if self.lazy[k] == A::id() {
            return;
        }

        self.data[k] =
            self.data[k].act(self.lazy[k].p(k.leading_zeros() - self.size.leading_zeros()));
        if !self.is_bottom(k) {
            self.lazy[2 * k + 1] = self.lazy[2 * k + 1].op(self.lazy[k]);
            self.lazy[2 * k + 2] = self.lazy[2 * k + 2].op(self.lazy[k]);
        }

        self.lazy[k] = A::id();
    }

    fn update_sub(&mut self, a: usize, b: usize, value: A, k: usize, left: usize, right: usize) {
        self.eval(k);

        if b <= left || right <= a {
            return;
        }

        if a <= left && right <= b {
            self.lazy[k] = self.lazy[k].op(value);
            self.eval(k);
            return;
        }

        let mid = (left + right) / 2;
        self.update_sub(a, b, value, 2 * k + 1, left, mid);
        self.update_sub(a, b, value, 2 * k + 2, mid, right);
        self.data[k] = self.data[2 * k + 1].op(self.data[2 * k + 2]);
    }

    pub fn update(&mut self, left: usize, right: usize, value: A) {
        self.update_sub(left, right, value, 0, 0, self.size);
    }

    fn get_sub(&mut self, a: usize, b: usize, k: usize, left: usize, right: usize) -> M {
        if right <= a || b <= left {
            return M::id();
        }

        self.eval(k);

        if a <= left && right <= b {
            return self.data[k];
        }

        let mid = (left + right) / 2;
        let result_left = self.get_sub(a, b, 2 * k + 1, left, mid);
        let result_right = self.get_sub(a, b, 2 * k + 2, mid, right);

        result_left.op(result_right)
    }

    pub fn get(&mut self, left: usize, right: usize) -> M {
        self.get_sub(left, right, 0, 0, self.size)
    }
}

#[cfg(test)]
mod tests {
    extern crate rstest;
    extern crate speculate;

    use std::fmt::Display;

    use rstest::*;
    use speculate::speculate;

    use crate::math::algebra::Semigroup;

    use super::*;

    speculate! {
        describe "LazySegmentTree to get the minimum value of a segment" {
            #[derive(Clone, Copy, PartialEq, Debug)]
            struct MinNum(i32);

            impl Semigroup for MinNum {
                fn op(&self, rhs: Self) -> Self {
                    Self(i32::min(self.0, rhs.0))
                }
            }

            impl Monoid for MinNum {
                fn id() -> Self {
                    Self(i32::MAX)
                }
            }

            impl ActOpMonoid<Self> for MinNum {
                fn act(&self, rhs: Self) -> Self {
                    self.op(rhs)
                }
            }

            impl SegmentPropertation for MinNum {
                fn p(&self, _: u32) -> Self {
                    *self
                }
            }

            impl Display for MinNum {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.fmt(f)
                }
            }

            describe "initialize" {
                #[rstest]
                fn test_initialize_with_new() {
                    let lst = LazySegmentTree::<MinNum, MinNum>::new(
                        10,
                    );
                    assert_eq!(lst.data.len(), 31);

                    for d in lst.data {
                        assert_eq!(d.0, i32::MAX);
                    }
                }

                #[rstest]
                fn test_initialize_with_from() {
                    let lst = LazySegmentTree::<MinNum, MinNum>::from(
                        &vec![1, 2, 3, 4, 5].iter().map(|n| MinNum(*n)).collect(),
                    );
                    assert_eq!(lst.data.len(), 15);

                    assert_eq!(lst.data, vec![
                        1, // root node (1st level)
                        1, 5, // (2nd level)
                        1, 3, 5, i32::MAX, // (3rd level)
                        1, 2, 3, 4, 5, i32::MAX, i32::MAX, i32::MAX // leaf nodes
                    ].iter().map(|n| MinNum(*n)).collect::<Vec<_>>());
                }
            }

            describe "update values if the incoming value is smaller" {
                #[rstest]
                fn test_update() {
                    let mut lst = LazySegmentTree::<MinNum, MinNum>::from(
                        &vec![1, 2, 3, 4, 5, 6, 7, 8].iter().map(|n| MinNum(*n)).collect(),
                    );
                    lst.update(3, 6, MinNum(2)); // data is [1, 2, 3, 2, 2, 2, 7, 8]

                    assert_eq!(lst.get(0, 1).0, 1);
                    assert_eq!(lst.get(1, 2).0, 2);
                    assert_eq!(lst.get(2, 3).0, 3);
                    assert_eq!(lst.get(3, 4).0, 2);
                    assert_eq!(lst.get(4, 5).0, 2);
                    assert_eq!(lst.get(5, 6).0, 2);
                    assert_eq!(lst.get(6, 7).0, 7);
                    assert_eq!(lst.get(7, 8).0, 8);
                    assert_eq!(lst.get(4, 7).0, 2);

                    lst.update(2, 4, MinNum(-1)); // data is [1, 2, -1, -1, 2, 2, 7, 8]
                    assert_eq!(lst.get(1, 2).0, 2);
                    assert_eq!(lst.get(2, 3).0, -1);
                    assert_eq!(lst.get(3, 4).0, -1);
                    assert_eq!(lst.get(4, 5).0, 2);

                    lst.update(0, 8, MinNum(0)); // data is [0, 0, -1, -1, 0, 0, 0, 0]
                    assert_eq!(lst.get(0, 4).0, -1);
                    assert_eq!(lst.get(2, 6).0, -1);
                }
            }
        }

        describe "LazySegmentTree to get the sum of a segment" {
            #[derive(Clone, Copy, PartialEq, Debug)]
            struct SumNum(i32);

            impl Semigroup for SumNum {
                fn op(&self, rhs: Self) -> Self {
                    Self(self.0 + rhs.0)
                }
            }

            impl Monoid for SumNum {
                fn id() -> Self {
                    Self(0)
                }
            }

            impl ActOpMonoid<SumNum> for SumNum {
                fn act(&self, rhs: SumNum) -> Self {
                    Self(self.0 + rhs.0)
                }
            }

            impl SegmentPropertation for SumNum {
                fn p(&self, len: u32) -> Self {
                    Self(self.0 << len)
                }
            }

            impl Display for SumNum {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.fmt(f)
                }
            }

            #[rstest]
            fn test_sum_of_get_segment() {
                let mut lst = LazySegmentTree::<SumNum, SumNum>::new(
                    16,
                );

                lst.update(3, 12, SumNum(1)); // data is [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0]
                assert_eq!(lst.get(0, 3).0, 0);
                assert_eq!(lst.get(4, 5).0, 1);
                assert_eq!(lst.get(8, 11).0, 3);

                lst.update(1, 5, SumNum(3)); // data is [0, 3, 3, 4, 4, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0]
                assert_eq!(lst.get(0, 1).0, 0);
                assert_eq!(lst.get(0, 2).0, 3);
                assert_eq!(lst.get(0, 3).0, 6);
                assert_eq!(lst.get(0, 4).0, 10);
                assert_eq!(lst.get(4, 5).0, 4);
                assert_eq!(lst.get(0, 3).0, 6);
                assert_eq!(lst.get(4, 11).0, 10);

                lst.update(9, 16, SumNum(-3)); // data is [0, 3, 3, 4, 4, 1, 1, 1, 1, -2, -2, -2, -3, -3, -3, -3]
                assert_eq!(lst.get(0, 1).0, 0);
                assert_eq!(lst.get(0, 2).0, 3);
                assert_eq!(lst.get(0, 9).0, 18);
                assert_eq!(lst.get(0, 10).0, 16);
                assert_eq!(lst.get(7, 16).0, -16);
            }
        }
    }
}
