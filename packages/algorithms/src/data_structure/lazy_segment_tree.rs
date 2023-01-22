struct LazySegmentTree {
    size: usize,
    data: Vec<i32>,
    lazy: Vec<i32>,
}

impl LazySegmentTree {
    pub fn new(size: usize) -> Self {
        let mut s = 1;
        while s < size {
            s *= 2;
        }

        LazySegmentTree {
            size: s,
            data: vec![i32::MAX; 2 * s - 1],
            lazy: vec![i32::MAX; 2 * s - 1],
        }
    }

    pub fn from(data: &Vec<i32>) -> Self {
        let mut s = 1;
        while s < data.len() {
            s *= 2;
        }

        let mut d = vec![i32::MAX; 2 * s - 1];

        for (i, x) in data.iter().enumerate() {
            d[s - 1 + i] = *x;
        }
        for i in (0..s - 1).rev() {
            d[i] = i32::min(d[i * 2 + 1], d[i * 2 + 2]);
        }

        LazySegmentTree {
            size: s,
            data: d,
            lazy: vec![i32::MAX; 2 * s - 1],
        }
    }

    fn is_bottom(&self, k: usize) -> bool {
        k >= self.size - 1
    }

    pub fn eval(&mut self, k: usize) {
        if self.lazy[k] == i32::MAX {
            return;
        }

        self.data[k] = i32::min(self.data[k], self.lazy[k]);
        if !self.is_bottom(k) {
            self.lazy[2 * k + 1] = i32::min(self.lazy[2 * k + 1], self.lazy[k]);
            self.lazy[2 * k + 2] = i32::min(self.lazy[2 * k + 2], self.lazy[k]);
        }

        self.lazy[k] = i32::MAX;
    }

    fn update_sub(&mut self, a: usize, b: usize, value: i32, k: usize, left: usize, right: usize) {
        self.eval(k);

        if b <= left || right <= a {
            return;
        }

        if a <= left && right <= b {
            self.lazy[k] = i32::min(self.lazy[k], value);
            self.eval(k);
            return;
        }

        let mid = (left + right) / 2;
        self.update_sub(a, b, value, 2 * k + 1, left, mid);
        self.update_sub(a, b, value, 2 * k + 2, mid, right);
        self.data[k] = i32::min(self.data[2 * k + 1], self.data[2 * k + 2]);
    }

    pub fn update(&mut self, left: usize, right: usize, value: i32) {
        self.update_sub(left, right, value, 0, 0, self.size);
    }

    fn get_sub(&mut self, a: usize, b: usize, k: usize, left: usize, right: usize) -> i32 {
        if right <= a || b <= left {
            return i32::MAX;
        }

        self.eval(k);

        if a <= left && right <= b {
            return self.data[k];
        }

        let mid = (left + right) / 2;
        let result_left = self.get_sub(a, b, 2 * k + 1, left, mid);
        let result_right = self.get_sub(a, b, 2 * k + 2, mid, right);

        i32::min(result_left, result_right)
    }

    pub fn get(&mut self, left: usize, right: usize) -> i32 {
        self.get_sub(left, right, 0, 0, self.size)
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
        describe "LazySegmentTree to get the minimum value of a segment" {
            describe "initialize" {
                #[rstest]
                fn test_initialize_with_new() {
                    let lst = LazySegmentTree::new(10);
                    assert_eq!(lst.data.len(), 31);

                    for d in lst.data {
                        assert_eq!(d, i32::MAX);
                    }
                }

                #[rstest]
                fn test_initialize_with_from() {
                    let lst = LazySegmentTree::from(&vec![1, 2, 3, 4, 5]);
                    assert_eq!(lst.data.len(), 15);

                    assert_eq!(lst.data, vec![
                        1, // root node (1st level)
                        1, 5, // (2nd level)
                        1, 3, 5, i32::MAX, // (3rd level)
                        1, 2, 3, 4, 5, i32::MAX, i32::MAX, i32::MAX // leaf nodes
                    ]);
                }
            }

            describe "update values if the incoming value is smaller" {
                #[rstest]
                fn test_update() {
                    let mut lst = LazySegmentTree::from(&vec![1, 2, 3, 4, 5, 6, 7, 8]);
                    lst.update(3, 6, 2); // data is [1, 2, 3, 2, 2, 2, 7, 8]

                    assert_eq!(lst.get(0, 1), 1);
                    assert_eq!(lst.get(1, 2), 2);
                    assert_eq!(lst.get(2, 3), 3);
                    assert_eq!(lst.get(3, 4), 2);
                    assert_eq!(lst.get(4, 5), 2);
                    assert_eq!(lst.get(5, 6), 2);
                    assert_eq!(lst.get(6, 7), 7);
                    assert_eq!(lst.get(7, 8), 8);
                    assert_eq!(lst.get(4, 7), 2);

                    lst.update(2, 4, -1); // data is [1, 2, -1, -1, 2, 2, 7, 8]
                    assert_eq!(lst.get(1, 2), 2);
                    assert_eq!(lst.get(2, 3), -1);
                    assert_eq!(lst.get(3, 4), -1);
                    assert_eq!(lst.get(4, 5), 2);

                    lst.update(0, 8, 0); // data is [0, 0, -1, -1, 0, 0, 0, 0]
                    assert_eq!(lst.get(0, 4), -1);
                    assert_eq!(lst.get(2, 6), -1);
                }
            }
        }
    }
}
