use std::collections::BinaryHeap;

use super::structs::UndirectedGraph;

pub struct Dijkstra {
    pub distances: Vec<usize>,
    pub parents: Vec<usize>,
}

impl Dijkstra {
    const INFINITY: usize = usize::MAX;
}

impl UndirectedGraph {
    pub fn dijkstra(self, start: usize) -> Dijkstra {
        let mut heap = BinaryHeap::new();
        let mut distances = vec![Dijkstra::INFINITY; self.adjacents.len()];
        let mut parents = vec![usize::MAX; self.adjacents.len()];

        heap.push((0 as i32, start, start));

        while !heap.is_empty() {
            if let Some((distance_negative, from, to)) = heap.pop() {
                let distance = distance_negative.abs() as usize;
                if distance >= distances[to] {
                    continue;
                }
                distances[to] = distance;
                parents[to] = from;

                for adj in &self.adjacents[to] {
                    // dbg!(-((distance + adj.distance) as i32), to, adj.node);
                    heap.push((-((distance + adj.distance) as i32), to, adj.node));
                }
            } else {
                panic!("heap is empty");
            }
        }

        Dijkstra { distances, parents }
    }
}

#[cfg(test)]
mod tests {
    extern crate rstest;
    extern crate speculate;

    use rstest::*;
    use speculate::speculate;

    use crate::graph::structs::Edge;

    use super::*;

    speculate! {
        describe "Dijkstra" {
            #[rstest]
            fn test_distance_caluculate() {
                let edges = vec![
                    Edge { left: 0, right: 1, distance: 7 },
                    Edge { left: 0, right: 2, distance: 4 },
                    Edge { left: 0, right: 3, distance: 3 },
                    Edge { left: 1, right: 2, distance: 1 },
                    Edge { left: 1, right: 4, distance: 2 },
                    Edge { left: 2, right: 4, distance: 6 },
                    Edge { left: 3, right: 4, distance: 5 },
                ];
                let graph = UndirectedGraph::new(5, &edges);

                let result = graph.dijkstra(0);

                assert_eq!(result.distances, [0, 5, 4, 3, 7]);
                assert_eq!(result.parents, [0, 2, 0, 0, 1]);
            }
        }
    }
}
