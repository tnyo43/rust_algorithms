pub struct Edge {
    pub left: usize,
    pub right: usize,
    pub distance: usize,
}

pub struct Adjacent {
    pub node: usize,
    pub distance: usize,
}

pub struct Graph<const DIRECTED: bool> {
    pub adjacents: Vec<Vec<Adjacent>>,
}

pub type UndirectedGraph = Graph<false>;
pub type DirectedGraph = Graph<true>;

impl<const DIRECTED: bool> Graph<DIRECTED> {
    pub fn new(nodes: usize, edges: &Vec<Edge>) -> Self {
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

        Graph { adjacents }
    }

    pub fn add(&mut self, edge: &Edge) {
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
