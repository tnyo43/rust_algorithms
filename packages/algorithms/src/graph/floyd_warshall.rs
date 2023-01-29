use crate::graph::structs::{Distance, Graph};

pub struct FloydWarshall<D>
where
    D: Distance,
{
    pub distances: Vec<Vec<D>>,
}

impl<D, const DIRECTED: bool> Graph<D, DIRECTED>
where
    D: Distance,
{
    pub fn floyd_warshall(self) -> FloydWarshall<D> {
        let mut distances = vec![vec![D::infinity(); self.nodes]; self.nodes];
        for node in 0..self.nodes {
            distances[node][node] = D::zero();

            for adj in &self.adjacents[node] {
                distances[node][adj.node] = adj.distance;
            }
        }

        for k in 0..self.nodes {
            for i in 0..self.nodes {
                for j in 0..self.nodes {
                    let d = distances[i][k].add(distances[k][j]);
                    if d < distances[i][j] {
                        distances[i][j] = d;
                    }
                }
            }
        }

        FloydWarshall { distances }
    }
}
