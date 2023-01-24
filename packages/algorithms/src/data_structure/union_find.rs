pub struct UnionFind {
    roots: Vec<usize>,
    ranks: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        let mut roots = vec![0; size];
        for i in 0..size {
            roots[i] = i;
        }

        UnionFind {
            roots,
            ranks: vec![0; size],
        }
    }

    pub fn root(&mut self, x: usize) -> usize {
        if self.roots[x] == x {
            return x;
        } else {
            let r = self.root(self.roots[x]);
            self.roots[x] = r;
            return r;
        }
    }

    pub fn merge(&mut self, x: usize, y: usize) {
        let x = self.root(x);
        let y = self.root(y);

        if x == y {
            return;
        }

        if self.ranks[x] < self.ranks[y] {
            self.roots[x] = y;
        } else {
            self.roots[y] = x;
            if self.ranks[x] == self.ranks[y] {
                self.ranks[x] += 1;
            }
        }
    }

    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
}

#[cfg(test)]
mod tests {
    extern crate rstest;
    extern crate speculate;

    use rstest::*;
    use speculate::speculate;

    use super::*;

    speculate! {
        describe "Union Find tree" {
            #[rstest]
            fn test_mege_is_same() {
                let mut uf = UnionFind::new(10);
                assert!(!uf.is_same(1, 3));

                uf.merge(1, 3);
                assert!(uf.is_same(1, 3));

                assert!(!uf.is_same(5, 8));
                uf.merge(5, 6);
                uf.merge(6, 7);
                uf.merge(7, 8);
                assert!(uf.is_same(5, 8));
            }
        }
    }
}
