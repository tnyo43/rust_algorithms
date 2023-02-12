// problem: https://atcoder.jp/contests/abc205/tasks/abc205_f
// answer: https://atcoder.jp/contests/abc205/submissions/38846615

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
    input! { h: usize, w: usize, n: usize };

    let mut graph = Graph::<C, true>::new(2 + h + w + 2 * n, &vec![]);
    // 0: start
    // 1: end
    // 2~h+1: rows
    // h+2~h+w+1: columns
    // h+w+2,h+w+4,h+w+6,...,h+w+2n: obj (row-side)
    // h+w+3,h+w+5,h+w+7,...,h+w+2n+1: obj (column-side)

    let unit = C(1);

    for i in 0..h {
        graph.add(&Edge {
            left: 0,
            right: i + 2,
            data: unit,
        });
    }

    for i in 0..n {
        let obj_row = 2 * (i + 1) + h + w;
        let obj_column = obj_row + 1;

        input! { a: usize, b: usize, c: usize, d: usize };

        for r in (a - 1)..=(c - 1) {
            graph.add(&Edge {
                left: r + 2,
                right: obj_row,
                data: unit,
            })
        }

        graph.add(&Edge {
            left: obj_row,
            right: obj_column,
            data: unit,
        });

        for col in (b - 1)..=(d - 1) {
            graph.add(&Edge {
                left: obj_column,
                right: col + h + 2,
                data: unit,
            })
        }
    }

    for i in 0..w {
        graph.add(&Edge {
            left: i + h + 2,
            right: 1,
            data: unit,
        });
    }

    println!("{}", graph.fort_fulkerson_max_flow(0, 1).0);
}
