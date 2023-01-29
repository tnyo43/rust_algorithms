pub trait Distance: Ord + Copy + Clone {
    fn zero() -> Self;
    fn infinity() -> Self;
    fn add(&self, rhs: Self) -> Self;
}

pub struct Edge<Data> {
    pub left: usize,
    pub right: usize,
    pub data: Data,
}

pub struct Adjacent<Data> {
    pub node: usize,
    pub data: Data,
}

pub struct Graph<V, const DIRECTED: bool> {
    pub nodes: usize,
    pub adjacents: Vec<Vec<Adjacent<V>>>,
}

impl<D, const DIRECTED: bool> Graph<D, DIRECTED>
where
    D: Distance,
{
    pub fn new(nodes: usize, edges: &Vec<Edge<D>>) -> Self {
        let mut adjacents = Vec::new();
        for _ in 0..nodes {
            adjacents.push(Vec::new());
        }

        for e in edges {
            adjacents[e.left].push(Adjacent {
                node: e.right,
                data: e.data,
            });

            if !DIRECTED {
                adjacents[e.right].push(Adjacent {
                    node: e.left,
                    data: e.data,
                });
            }
        }

        Graph { nodes, adjacents }
    }

    pub fn add(&mut self, edge: &Edge<D>) {
        self.adjacents[edge.left].push(Adjacent {
            node: edge.right,
            data: edge.data,
        });

        if !DIRECTED {
            self.adjacents[edge.right].push(Adjacent {
                node: edge.left,
                data: edge.data,
            });
        }
    }
}
