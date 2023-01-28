pub struct Edge {
    left: usize,
    right: usize,
    distance: usize,
}

pub struct Adjacent {
    pub node: usize,
    pub distance: usize,
}

pub struct UndirectedGraph {
    pub adjacents: Vec<Vec<Adjacent>>,
}

impl UndirectedGraph {
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
            adjacents[e.right].push(Adjacent {
                node: e.left,
                distance: e.distance,
            });
        }

        UndirectedGraph { adjacents }
    }

    pub fn add(&mut self, edge: &Edge) {
        self.adjacents[edge.left].push(Adjacent {
            node: edge.right,
            distance: edge.distance,
        });
        self.adjacents[edge.right].push(Adjacent {
            node: edge.left,
            distance: edge.distance,
        });
    }
}
