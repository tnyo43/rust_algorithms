pub trait Distance: Ord + Copy + Clone {
    fn zero() -> Self;
    fn infinity() -> Self;
    fn add(&self, rhs: Self) -> Self;
}

pub struct Edge<D>
where
    D: Distance,
{
    pub left: usize,
    pub right: usize,
    pub distance: D,
}

pub struct Adjacent<D>
where
    D: Distance,
{
    pub node: usize,
    pub distance: D,
}

pub struct Graph<D, const DIRECTED: bool>
where
    D: Distance,
{
    pub nodes: usize,
    pub adjacents: Vec<Vec<Adjacent<D>>>,
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
                distance: e.distance,
            });

            if !DIRECTED {
                adjacents[e.right].push(Adjacent {
                    node: e.left,
                    distance: e.distance,
                });
            }
        }

        Graph { nodes, adjacents }
    }

    pub fn add(&mut self, edge: &Edge<D>) {
        self.adjacents[edge.left].push(Adjacent {
            node: edge.right,
            distance: edge.distance,
        });

        if !DIRECTED {
            self.adjacents[edge.right].push(Adjacent {
                node: edge.left,
                distance: edge.distance,
            });
        }
    }
}
