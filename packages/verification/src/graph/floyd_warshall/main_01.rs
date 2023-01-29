// problems: https://atcoder.jp/contests/abc079/tasks/abc079_d
// answer: https://atcoder.jp/contests/abc079/submissions/38416105

use algorithms::graph::structs::{Distance, Edge, Graph};
use proconio::input;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct D(usize);

impl Distance for D {
    fn zero() -> Self {
        D(0)
    }

    fn infinity() -> Self {
        D(usize::MAX)
    }

    fn add(&self, rhs: Self) -> Self {
        D(self.0 + rhs.0)
    }
}

fn main() {
    input! { h: usize, w: usize };

    let mut graph = Graph::<D, true>::new(10, &Vec::new());
    for i in 0..=9 {
        for j in 0..=9 {
            input! { c: usize };
            graph.add(&Edge::<D> {
                left: i,
                right: j,
                distance: D(c),
            })
        }
    }

    let result = graph.floyd_warshall();

    let mut ans = 0;
    for _ in 0..(h * w) {
        input! { a: i8 };
        if a == -1 {
            continue;
        }
        ans += result.distances[a as usize][1].0;
    }

    println!("{}", ans);
}
