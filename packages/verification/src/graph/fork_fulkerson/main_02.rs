// problem: https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_bp
// answer: https://atcoder.jp/contests/tessoku-book/submissions/38846991

use std::ops;

use algorithms::graph::{
    fort_fulkerson::Capacity,
    structs::{Edge, Graph},
};
use proconio::input;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct C(usize);

impl Capacity for C {
    fn zero() -> Self {
        C(0)
    }

    fn infinity() -> Self {
        C(usize::MAX)
    }
}

impl ops::AddAssign for C {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0)
    }
}

impl ops::SubAssign for C {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self(self.0 - rhs.0)
    }
}

fn main() {
    input! { n: usize, m: usize };

    let mut graph = Graph::<C, true>::new(n, &vec![]);

    for _ in 0..m {
        input! { a: usize, b: usize, c: usize };
        graph.add(&Edge {
            left: a - 1,
            right: b - 1,
            data: C(c),
        })
    }

    println!("{}", graph.fort_fulkerson_max_flow(0, n - 1).0);
}
