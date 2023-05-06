use super::structs::Graph;

const COMP_EMPTY: usize = usize::MAX;

impl<T> Graph<T, true> {
    fn dfs(
        &self,
        u: usize,
        tm: &mut i32,
        cnt: &mut usize,
        ord: &mut Vec<i32>,
        low: &mut Vec<i32>,
        comp: &mut Vec<usize>,
        stack: &mut Vec<usize>,
    ) {
        ord[u] = *tm;
        low[u] = *tm;
        stack.push(u);
        *tm += 1;

        for adj in &self.adjacents[u] {
            if ord[adj.node] < 0 {
                self.dfs(adj.node, tm, cnt, ord, low, comp, stack);
                low[u] = i32::min(low[u], low[adj.node]);
            } else if comp[adj.node] == COMP_EMPTY {
                low[u] = i32::min(low[u], ord[adj.node]);
            }
        }
        if ord[u] == low[u] {
            while let Some(v) = stack.pop() {
                comp[v] = *cnt;
                if v == u {
                    break;
                }
            }
            *cnt += 1;
        }
    }

    pub fn strongly_connected_component(self) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
        let mut tm = 0;
        let mut cnt = 0;
        let mut ord = vec![-1; self.nodes];
        let mut low = vec![-1; self.nodes];
        let mut comp = vec![COMP_EMPTY; self.nodes];
        let mut stack = vec![];

        for i in 0..self.nodes {
            if ord[i] < 0 {
                self.dfs(
                    i, &mut tm, &mut cnt, &mut ord, &mut low, &mut comp, &mut stack,
                );
            }
        }

        for i in 0..self.nodes {
            comp[i] = cnt - 1 - comp[i];
        }

        let mut scc = vec![Vec::new(); cnt];
        let mut graph = vec![Vec::new(); cnt];
        for i in 0..self.nodes {
            scc[comp[i]].push(i);

            for adj in self.adjacents[i].iter() {
                if comp[i] != comp[adj.node] {
                    graph[comp[i]].push(comp[adj.node]);
                }
            }
        }

        for i in 0..cnt {
            graph[i].sort();
            graph[i].dedup();
        }

        (scc, graph)
    }
}
