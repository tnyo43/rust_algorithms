use std::fmt::Display;

pub struct LazySegmentTree<D, L, F, G, H, I, J>
where
    D: Clone + Copy + Display,
    L: Clone + Copy + PartialEq + Display,
    F: Fn() -> D,
    G: Fn() -> L,
    H: Fn(D, D) -> D,
    I: Fn(D, L) -> D,
    J: Fn(L, L) -> L,
{
    size: usize,
    data: Vec<D>,
    lazy: Vec<L>,
    zero_data: F,
    zero_lazy: G,
    op: H,
    mapping: I,
    composite: J,
}

fn print<T: Display>(
    f: &mut std::fmt::Formatter<'_>,
    data: &Vec<T>,
    size: usize,
) -> std::fmt::Result {
    for (i, v) in data.iter().enumerate() {
        let n = 1 << ((i + 1).leading_zeros() - size.leading_zeros());

        for _ in 0..(n / 2) {
            write!(f, "\t")?;
        }
        write!(f, "{}", v)?;
        for _ in 0..((n + 1) / 2) {
            write!(f, "\t")?;
        }

        if (i + 2).count_ones() == 1 {
            write!(f, "\n")?;
        }
    }
    write!(f, "")
}

impl<D, L, F, G, H, I, J> Display for LazySegmentTree<D, L, F, G, H, I, J>
where
    D: Clone + Copy + Display,
    L: Clone + Copy + PartialEq + Display,
    F: Fn() -> D,
    G: Fn() -> L,
    H: Fn(D, D) -> D,
    I: Fn(D, L, usize, u32) -> D,
    J: Fn(L, L) -> L,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[[data]]\n")?;
        print(f, &self.data, self.size)?;
        write!(f, "\n[[lazy]]\n")?;
        print(f, &self.lazy, self.size)?;
        write!(f, "")
    }
}

impl<D, L, F, G, H, I, J> LazySegmentTree<D, L, F, G, H, I, J>
where
    D: Clone + Copy,
    L: Clone + Copy + PartialEq,
    F: Fn() -> D,
    G: Fn() -> L,
    H: Fn(D, D) -> D,
    I: Fn(D, L) -> D,
    J: Fn(L, L) -> L,
{
    pub fn new(size: usize, zero_data: F, zero_lazy: G, op: H, mapping: I, composite: J) -> Self {
        let mut s = 1;
        while s < size {
            s *= 2;
        }

        LazySegmentTree::<D, L, F, G, H, I, J> {
            size: s,
            data: vec![zero_data(); 2 * s - 1],
            lazy: vec![zero_lazy(); 2 * s - 1],
            zero_data,
            zero_lazy,
            op,
            mapping,
            composite,
        }
    }

    pub fn from(
        data: &Vec<D>,
        zero_data: F,
        zero_lazy: G,
        op: H,
        mapping: I,
        composite: J,
    ) -> Self {
        let mut s = 1;
        while s < data.len() {
            s *= 2;
        }

        let mut d = vec![zero_data(); 2 * s - 1];

        for (i, x) in data.iter().enumerate() {
            d[s - 1 + i] = x.clone();
        }
        for i in (0..s - 1).rev() {
            d[i] = op(d[i * 2 + 1], d[i * 2 + 2]);
        }

        LazySegmentTree::<D, L, F, G, H, I, J> {
            size: s,
            data: d,
            lazy: vec![zero_lazy(); 2 * s - 1],
            zero_data,
            zero_lazy,
            op,
            mapping,
            composite,
        }
    }

    fn is_bottom(&self, k: usize) -> bool {
        k >= self.size - 1
    }

    pub fn eval(&mut self, k: usize) {
        if self.lazy[k] == (self.zero_lazy)() {
            return;
        }

        self.data[k] = (self.mapping)(self.data[k], self.lazy[k]);
        if !self.is_bottom(k) {
            self.lazy[2 * k + 1] = (self.composite)(self.lazy[2 * k + 1], self.lazy[k]);
            self.lazy[2 * k + 2] = (self.composite)(self.lazy[2 * k + 2], self.lazy[k]);
        }

        self.lazy[k] = (self.zero_lazy)();
    }

    fn update_sub(&mut self, a: usize, b: usize, value: L, k: usize, left: usize, right: usize) {
        self.eval(k);

        if b <= left || right <= a {
            return;
        }

        if a <= left && right <= b {
            self.lazy[k] = (self.composite)(self.lazy[k], value);
            self.eval(k);
            return;
        }

        let mid = (left + right) / 2;
        self.update_sub(a, b, value, 2 * k + 1, left, mid);
        self.update_sub(a, b, value, 2 * k + 2, mid, right);
        self.data[k] = (self.op)(self.data[2 * k + 1], self.data[2 * k + 2]);
    }

    pub fn update(&mut self, left: usize, right: usize, value: L) {
        self.update_sub(left, right, value, 0, 0, self.size);
    }

    fn get_sub(&mut self, a: usize, b: usize, k: usize, left: usize, right: usize) -> D {
        if right <= a || b <= left {
            return (self.zero_data)();
        }

        self.eval(k);

        if a <= left && right <= b {
            return self.data[k];
        }

        let mid = (left + right) / 2;
        let result_left = self.get_sub(a, b, 2 * k + 1, left, mid);
        let result_right = self.get_sub(a, b, 2 * k + 2, mid, right);

        (self.op)(result_left, result_right)
    }

    pub fn get(&mut self, left: usize, right: usize) -> D {
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
                    let lst = LazySegmentTree::new(
                        10,
                        || { i32::MAX },
                        || { i32::MAX },
                        i32::min,
                        i32::min,
                        i32::min
                    );
                    assert_eq!(lst.data.len(), 31);

                    for d in lst.data {
                        assert_eq!(d, i32::MAX);
                    }
                }

                #[rstest]
                fn test_initialize_with_from() {
                    let lst = LazySegmentTree::from(
                        &vec![1, 2, 3, 4, 5],
                        || { i32::MAX },
                        || { i32::MAX },
                        i32::min,
                        i32::min,
                        i32::min
                    );
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
                    let mut lst = LazySegmentTree::from(
                        &vec![1, 2, 3, 4, 5, 6, 7, 8],
                        || { i32::MAX },
                        || { i32::MAX },
                        i32::min,
                        i32::min,
                        i32::min
                    );
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
