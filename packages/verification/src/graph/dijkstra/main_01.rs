// problem: https://atcoder.jp/contests/abc068/tasks/arc079_a
// answer: https://atcoder.jp/contests/abc068/submissions/38389856

use algorithms::graph::structs::{Distance, Edge, Graph};
use proconio::input;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct D(usize);

impl Distance for D {
    fn zero() -> Self {
        Self(0)
    }

    fn infinity() -> Self {
        Self(usize::MAX)
    }

    fn add(&self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

fn main() {
    input! { n: usize, m: usize };
    let mut graph = Graph::<D, false>::new(n, &vec![]);

    for _ in 0..m {
        input! { a: usize, b: usize };
        graph.add(&Edge {
            left: a - 1,
            right: b - 1,
            distance: D(1),
        });
    }

    let result = graph.dijkstra(0);
    if result.distances[n - 1].0 == 2 {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
    }
}
