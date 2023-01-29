// problem: https://atcoder.jp/contests/abc257/tasks/abc257_d
// answer: https://atcoder.jp/contests/abc257/submissions/38439376

use algorithms::graph::structs::{Distance, Edge, Graph};
use proconio::input;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct D(usize);

impl Distance for D {
    fn zero() -> Self {
        D(0)
    }

    fn infinity() -> Self {
        D(usize::MAX >> 2)
    }

    fn add(&self, rhs: Self) -> Self {
        D(self.0 + rhs.0)
    }
}

fn check(v: &Vec<(i64, i64, u64)>, s: u64) -> bool {
    let mut g = Graph::<D, true>::new(v.len(), &vec![]);
    for i in 0..v.len() {
        for j in 0..v.len() {
            let x = (v[i].0 - v[j].0).abs();
            let y = (v[i].1 - v[j].1).abs();
            if v[i].2 * s >= (x + y) as u64 {
                g.add(&Edge {
                    left: i,
                    right: j,
                    distance: D(0),
                });
            }
        }
    }

    let result = g.floyd_warshall();

    for i in 0..v.len() {
        let mut tmp = true;
        for j in 0..v.len() {
            if result.distances[i][j] == D::infinity() {
                tmp = false;
                break;
            }
        }
        if tmp {
            return true;
        }
    }

    false
}

fn main() {
    input! { n: usize };

    let mut v = Vec::new();
    for _ in 0..n {
        input! { x: i64, y: i64, p: u64 };
        v.push((x, y, p));
    }

    let mut ok: u64 = 4_100_000_000;
    let mut ng = 0;

    while ok - ng > 1 {
        let mid = (ok + ng) / 2;
        if check(&v, mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
