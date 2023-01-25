// problem: https://atcoder.jp/contests/practice2/tasks/practice2_k
// answer: https://atcoder.jp/contests/practice2/submissions/38324669

use std::fmt::Display;

use algorithms::{
    data_structure::lazy_segment_tree::{LazySegmentTree, SegmentPropertation},
    math::{
        algebra::{ActOpMonoid, Monoid, Semigroup},
        modint::Modint,
    },
};
use proconio::input;

type M = Modint<998244353>;

#[derive(Clone, Copy, PartialEq)]
struct Lazy {
    b: M,
    c: M,
}

impl Semigroup for Lazy {
    fn op(&self, rhs: Self) -> Self {
        Self {
            b: self.b * rhs.b,
            c: self.c * rhs.b + rhs.c,
        }
    }
}

impl Monoid for Lazy {
    fn id() -> Self {
        Self {
            b: M::new(1),
            c: M::new(0),
        }
    }
}

impl SegmentPropertation for Lazy {
    fn p(&self, len: u32) -> Self {
        Self {
            b: self.b,
            c: self.c * M::new(len as usize),
        }
    }
}

impl Display for Lazy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.b, self.c)
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Data(M);

impl Semigroup for Data {
    fn op(&self, rhs: Self) -> Self {
        Data(self.0 + rhs.0)
    }
}

impl Monoid for Data {
    fn id() -> Self {
        Data(Modint::new(0))
    }
}

impl ActOpMonoid<Lazy> for Data {
    fn act(&self, rhs: Lazy) -> Self {
        Self(self.0 * rhs.b + rhs.c)
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

fn main() {
    input! { n: usize, q: usize };

    let mut a = Vec::new();
    for _ in 0..n {
        input! { _a: usize };
        a.push(Data(M::new(_a)));
    }

    let mut tree = LazySegmentTree::<Data, Lazy>::from(&a);

    for _ in 0..q {
        input! { f: u8 };

        if f == 0 {
            input! { l: usize, r: usize, b: usize, c: usize };
            tree.update(
                l,
                r,
                Lazy {
                    b: M::new(b),
                    c: M::new(c),
                },
            )
        } else {
            input! { l: usize, r: usize };
            println!("{}", tree.get(l, r));
        }
    }
}
