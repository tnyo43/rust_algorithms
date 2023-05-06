// problem: https://atcoder.jp/contests/abc245/tasks/abc245_f
// answer: https://atcoder.jp/contests/abc245/submissions/40357453

use algorithms::graph::structs::{Edge, Graph};
use proconio::input;

fn can_loop(i: usize, scc_status: &mut Vec<Option<bool>>, scc_graph: &Vec<Vec<usize>>) -> bool {
    if let Some(can_loop_i) = scc_status[i] {
        return can_loop_i;
    } else {
        scc_status[i] = Some(false);
        for &adj in &scc_graph[i] {
            if can_loop(adj, scc_status, scc_graph) {
                scc_status[i] = Some(true);
                return true;
            }
        }
    }

    return false;
}

fn main() {
    input! { n: usize, m: usize };
    let mut graph = Graph::new(n, &vec![]);

    for _ in 0..m {
        input! { u: usize, v: usize };
        graph.add(&Edge {
            left: u - 1,
            right: v - 1,
            data: (),
        });
    }

    let (scc, scc_graph) = graph.strongly_connected_component();

    let mut scc_status = scc
        .iter()
        .map(|group| if group.len() == 1 { None } else { Some(true) })
        .collect();

    let mut ans = 0;
    for i in 0..scc.len() {
        if can_loop(i, &mut scc_status, &scc_graph) {
            ans += scc[i].len();
        }
    }

    println!("{}", ans);
}
