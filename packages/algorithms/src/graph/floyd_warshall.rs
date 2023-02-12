use crate::graph::structs::{Distance, Graph};

pub struct FloydWarshall<Data>
where
    Data: Distance,
{
    pub distances: Vec<Vec<Data>>,
}

impl<Data, const DIRECTED: bool> Graph<Data, DIRECTED>
where
    Data: Distance,
{
    pub fn floyd_warshall(self) -> FloydWarshall<Data> {
        let mut distances = vec![vec![Data::infinity(); self.nodes]; self.nodes];
        for node in 0..self.nodes {
            distances[node][node] = Data::zero();

            for adj in &self.adjacents[node] {
                distances[node][adj.node] = adj.data;
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
