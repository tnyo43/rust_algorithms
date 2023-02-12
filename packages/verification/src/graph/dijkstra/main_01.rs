// problem: https://atcoder.jp/contests/abc068/tasks/arc079_a
// answer: https://atcoder.jp/contests/abc068/submissions/38389856

use algorithms::graph::structs::{Distance, Edge, Graph};
use proconio::input;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Data {
    distance: usize,
}

impl Distance for Data {
    fn zero() -> Self {
        Data { distance: 0 }
    }

    fn infinity() -> Self {
        Data {
            distance: usize::MAX,
        }
    }

    fn add(&self, rhs: Self) -> Self {
        Data {
            distance: self.distance + rhs.distance,
        }
    }
}

fn main() {
    input! { n: usize, m: usize };
    let mut graph = Graph::<Data, false>::new(n, &vec![]);

    for _ in 0..m {
        input! { a: usize, b: usize };
        graph.add(&Edge {
            left: a - 1,
            right: b - 1,
            data: Data { distance: 1 },
        });
    }

    let result = graph.dijkstra(0);
    if result.distances[n - 1].distance == 2 {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
    }
}
