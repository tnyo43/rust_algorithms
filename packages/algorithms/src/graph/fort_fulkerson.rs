use std::ops;

use super::structs::Graph;

pub trait Capacity: Ord + Clone + Copy + ops::AddAssign + ops::SubAssign {
    fn zero() -> Self;
    fn infinity() -> Self;
}

#[derive(Clone, Copy)]
struct Edge<C>
where
    C: Capacity,
{
    to: usize,
    rev: usize,
    capacity: C,
}

struct FFGraph<C>
where
    C: Capacity,
{
    edges: Vec<Vec<Edge<C>>>,
    visited: Vec<bool>,
}

impl<C> FFGraph<C>
where
    C: Capacity,
{
    pub fn new(g: &Graph<C, true>) -> Self {
        let mut ret = FFGraph {
            edges: vec![vec![]; g.nodes],
            visited: vec![false; g.nodes],
        };

        for from in 0..g.nodes {
            for adj in &g.adjacents[from] {
                let e1_rev = ret.edges[adj.node].len();
                let e2_rev = ret.edges[from].len();

                ret.edges[from].push(Edge {
                    to: adj.node,
                    rev: e1_rev,
                    capacity: adj.data,
                });
                ret.edges[adj.node].push(Edge {
                    to: from,
                    rev: e2_rev,
                    capacity: Capacity::zero(),
                });
            }
        }

        ret
    }

    fn reset_visited(&mut self) {
        self.visited = vec![false; self.edges.len()];
    }

    fn dfs_fort_fulkerson(&mut self, from: usize, to: usize, flow: C) -> C {
        if from == to {
            return flow;
        }

        self.visited[from] = true;
        for i in 0..self.edges[from].len() {
            let edge = self.edges[from][i];
            if !self.visited[edge.to] && edge.capacity > C::zero() {
                let result = self.dfs_fort_fulkerson(edge.to, to, C::min(flow, edge.capacity));

                if result > C::zero() {
                    self.edges[from][i].capacity -= result;
                    self.edges[edge.to][edge.rev].capacity += result;
                    return result;
                }
            }
        }

        C::zero()
    }
}

impl<C> Graph<C, true>
where
    C: Capacity,
{
    pub fn fort_fulkerson_max_flow(&self, from: usize, to: usize) -> C {
        let mut ff_graph = FFGraph::new(self);

        let mut flow = C::zero();
        loop {
            ff_graph.reset_visited();
            let diff = ff_graph.dfs_fort_fulkerson(from, to, C::infinity());

            if diff == C::zero() {
                return flow;
            }

            flow += diff;
        }
    }
}
