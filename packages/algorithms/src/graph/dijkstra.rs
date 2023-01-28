use std::{cmp::Reverse, collections::BinaryHeap};

use crate::graph::structs::{Distance, Graph};

pub struct Dijkstra<D>
where
    D: Distance,
{
    pub distances: Vec<D>,
    pub parents: Vec<usize>,
}

impl<const DIRECTED: bool, D> Graph<D, DIRECTED>
where
    D: Distance,
{
    pub fn dijkstra(self, start: usize) -> Dijkstra<D> {
        let mut heap = BinaryHeap::new();
        let mut distances = vec![D::infinity(); self.adjacents.len()];
        let mut parents = vec![usize::MAX; self.adjacents.len()];

        heap.push(Reverse((D::zero(), start, start)));

        while !heap.is_empty() {
            if let Some(Reverse((distance, from, to))) = heap.pop() {
                if distance >= distances[to] {
                    continue;
                }
                distances[to] = distance;
                parents[to] = from;

                for adj in &self.adjacents[to] {
                    heap.push(Reverse((distance.add(adj.distance), to, adj.node)));
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

    type D = usize;

    impl Distance for D {
        fn zero() -> Self {
            0
        }

        fn infinity() -> Self {
            usize::MAX
        }

        fn add(&self, rhs: Self) -> Self {
            self + rhs
        }
    }

    speculate! {
        describe "Dijkstra" {
            #[rstest]
            fn test_distance_caluculate_of_undirected_graph() {
                let edges = vec![
                    Edge { left: 0, right: 1, distance: 7 },
                    Edge { left: 0, right: 2, distance: 4 },
                    Edge { left: 0, right: 3, distance: 3 },
                    Edge { left: 1, right: 2, distance: 1 },
                    Edge { left: 1, right: 4, distance: 2 },
                    Edge { left: 2, right: 4, distance: 6 },
                    Edge { left: 3, right: 4, distance: 5 },
                ];
                let graph = Graph::<D, false>::new(5, &edges);

                let result = graph.dijkstra(0);

                assert_eq!(result.distances, [0, 5, 4, 3, 7]);
                assert_eq!(result.parents, [0, 2, 0, 0, 1]);
            }

            #[rstest]
            fn test_distance_caluculate_of_directed_graph() {
                let edges = vec![
                    Edge { left: 0, right: 1, distance: 7 },
                    Edge { left: 0, right: 2, distance: 4 },
                    Edge { left: 0, right: 3, distance: 3 },
                    Edge { left: 1, right: 2, distance: 1 },
                    Edge { left: 1, right: 4, distance: 2 },
                    Edge { left: 2, right: 4, distance: 6 },
                    Edge { left: 3, right: 4, distance: 5 },
                ];
                let graph = Graph::<D, true>::new(5, &edges);

                let result = graph.dijkstra(0);

                assert_eq!(result.distances, [0, 7, 4, 3, 8]);
                assert_eq!(result.parents, [0, 0, 0, 0, 3]);
            }
        }
    }
}
