pub struct UndirectedGraph {
    adjacents: Vec<Vec<usize>>,
}

impl UndirectedGraph {
    pub fn new(nodes: usize, edges: &Vec<(usize, usize)>) -> Self {
        let mut adjacents = Vec::new();
        for _ in 0..nodes {
            adjacents.push(Vec::new());
        }

        for (a, b) in edges {
            adjacents[*a].push(*b);
            adjacents[*b].push(*a);
        }

        UndirectedGraph { adjacents }
    }

    pub fn add(&mut self, edge: (usize, usize)) {
        self.adjacents[edge.0].push(edge.1);
        self.adjacents[edge.1].push(edge.0);
    }
}
