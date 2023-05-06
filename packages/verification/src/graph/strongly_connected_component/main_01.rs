// problem: https://atcoder.jp/contests/abc256/tasks/abc256_e
// answer: https://atcoder.jp/contests/abc256/submissions/40357478

use algorithms::graph::structs::{Edge, Graph};
use proconio::input;

fn main() {
    input! { n: usize };
    let mut graph = Graph::new(n, &vec![]);

    let mut x = vec![0; n];
    let mut c = vec![0; n];
    for i in 0..n {
        input! { xx: usize };
        x[i] = xx - 1;
        graph.add(&Edge {
            left: i,
            right: x[i],
            data: (),
        })
    }
    for i in 0..n {
        input! {cc: usize};
        c[i] = cc;
    }

    let scc = graph.strongly_connected_component();
    let mut ans = 0;

    for group in scc.0 {
        if group.len() == 1 {
            continue;
        }
        ans += group.iter().map(|n| (c[*n])).min().unwrap();
    }

    println!("{}", ans);
}
