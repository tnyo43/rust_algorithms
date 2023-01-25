// problem: https://atcoder.jp/contests/practice2/tasks/practice2_j
// answer: https://atcoder.jp/contests/practice2/submissions/38323697

use std::fmt::Display;

use algorithms::{
    data_structure::lazy_segment_tree::{LazySegmentTree, SegmentPropertation},
    math::algebra::{ActOpMonoid, Monoid, Semigroup},
};
use proconio::input;

#[derive(Clone, Copy, PartialEq, Debug)]
struct MaxNum(i32);

impl Semigroup for MaxNum {
    fn op(&self, rhs: Self) -> Self {
        Self(i32::max(self.0, rhs.0))
    }
}

impl Monoid for MaxNum {
    fn id() -> Self {
        Self(i32::MIN)
    }
}

impl ActOpMonoid<Self> for MaxNum {
    fn act(&self, rhs: Self) -> Self {
        rhs
    }
}

impl SegmentPropertation for MaxNum {
    fn p(&self, _: u32) -> Self {
        *self
    }
}

impl Display for MaxNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

fn main() {
    input! { n: usize, q: usize };

    let mut a = Vec::new();

    a.push(MaxNum::id());
    for _ in 0..n {
        input! { _a: i32 };
        a.push(MaxNum(_a));
    }
    a.push(MaxNum(i32::MAX));
    a.push(MaxNum(i32::MAX));

    let mut max_segment_tree = LazySegmentTree::<MaxNum, MaxNum>::from(&a);

    for _ in 0..q {
        input! { t: i32 };

        if t == 1 {
            input! { x: usize, v: i32 };
            max_segment_tree.update(x, x + 1, MaxNum(v));
        } else if t == 2 {
            input! { l: usize, r: usize };
            println!("{}", max_segment_tree.get(l, r + 1));
        } else {
            input! { x: usize, v: i32 };

            let mut left = x;
            let mut right = n + 2;

            while right - left > 1 {
                let mid = (left + right) / 2;
                if max_segment_tree.get(left, mid).0 >= v {
                    right = mid;
                } else {
                    left = mid;
                }
            }
            println!("{}", left);
        }
    }
}
