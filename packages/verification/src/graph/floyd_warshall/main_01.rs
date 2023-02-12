// problems: https://atcoder.jp/contests/abc079/tasks/abc079_d
// answer: https://atcoder.jp/contests/abc079/submissions/38416105

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
    input! { h: usize, w: usize };

    let mut graph = Graph::<Data, true>::new(10, &Vec::new());
    for i in 0..=9 {
        for j in 0..=9 {
            input! { c: usize };
            graph.add(&Edge::<Data> {
                left: i,
                right: j,
                data: Data { distance: c },
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
        ans += result.distances[a as usize][1].distance;
    }

    println!("{}", ans);
}
