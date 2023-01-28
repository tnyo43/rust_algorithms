// problem: https://atcoder.jp/contests/abc192/tasks/abc192_e
// answer: https://atcoder.jp/contests/abc192/submissions/38383954

use algorithms::graph::structs::{Distance, Edge, Graph};
use proconio::input;

#[derive(PartialEq, Eq, Clone, Copy)]
struct D {
    time: usize,
    departure_every: usize,
}

impl PartialOrd for D {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.time.partial_cmp(&other.time)
    }
}

impl Ord for D {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.cmp(&other.time)
    }
}

impl Distance for D {
    fn zero() -> Self {
        Self {
            time: 0,
            departure_every: 1,
        }
    }

    fn infinity() -> Self {
        Self {
            time: usize::MAX,
            departure_every: usize::MAX,
        }
    }

    fn add(&self, rhs: Self) -> Self {
        let since_latest_departure = self.time % rhs.departure_every;

        let depart_at = if since_latest_departure == 0 {
            self.time
        } else {
            self.time - since_latest_departure + rhs.departure_every
        };

        Self {
            time: depart_at + rhs.time,
            departure_every: rhs.departure_every,
        }
    }
}

fn main() {
    input! { n: usize, m: usize, x: usize, y: usize };

    let mut graph = Graph::<D, true>::new(n, &Vec::new());
    for _ in 0..m {
        input! { a: usize, b: usize, t: usize, k: usize };

        graph.add(&Edge::<D> {
            left: a - 1,
            right: b - 1,
            distance: D {
                time: t,
                departure_every: k,
            },
        });
        graph.add(&Edge::<D> {
            left: b - 1,
            right: a - 1,
            distance: D {
                time: t,
                departure_every: k,
            },
        });
    }

    let result = graph.dijkstra(x - 1).distances[y - 1].time;
    if result == D::infinity().time {
        println!("-1");
    } else {
        println!("{}", result);
    }
}
