// problem: https://atcoder.jp/contests/practice2/tasks/practice2_l
// answer: https://atcoder.jp/contests/practice2/submissions/38323732

use algorithms::{
    data_structure::lazy_segment_tree::{LazySegmentTree, SegmentPropertation},
    math::algebra::{ActOpMonoid, Monoid, Semigroup},
};
use proconio::input;
use std::fmt::Display;

#[derive(Clone, Copy, PartialEq)]
struct Flip(bool);

impl Semigroup for Flip {
    fn op(&self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}

impl Monoid for Flip {
    fn id() -> Self {
        Self(false)
    }
}

impl Display for Flip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl SegmentPropertation for Flip {
    fn p(&self, _: u32) -> Self {
        *self
    }
}
#[derive(Clone, Copy)]
struct Data {
    inversion_number: usize,
    black: usize,
    white: usize,
}

impl Semigroup for Data {
    fn op(&self, rhs: Self) -> Self {
        Self {
            inversion_number: self.inversion_number + rhs.inversion_number + self.black * rhs.white,
            black: self.black + rhs.black,
            white: self.white + rhs.white,
        }
    }
}

impl Monoid for Data {
    fn id() -> Self {
        Self {
            inversion_number: 0,
            black: 0,
            white: 0,
        }
    }
}

impl ActOpMonoid<Flip> for Data {
    fn act(&self, rhs: Flip) -> Self {
        if rhs.0 {
            return Self {
                inversion_number: self.black * self.white - self.inversion_number,
                black: self.white,
                white: self.black,
            };
        } else {
            return *self;
        }
    }
}

impl Display for Data {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

fn main() {
    input! { n: usize, q: usize };

    let mut data = Vec::new();

    for _ in 0..n {
        input! { a: usize };
        data.push(Data {
            inversion_number: 0,
            black: a,
            white: 1 - a,
        });
    }

    let mut tree = LazySegmentTree::<Data, Flip>::from(&data);

    for _ in 0..q {
        input! { t: usize, l: usize, r: usize };

        if t == 1 {
            tree.update(l - 1, r, Flip(true));
        } else {
            println!("{}", tree.get(l - 1, r).inversion_number);
        }
    }
}
