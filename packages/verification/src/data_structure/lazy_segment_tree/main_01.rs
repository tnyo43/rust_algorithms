// problem: https://atcoder.jp/contests/practice2/tasks/practice2_j
// answer: https://atcoder.jp/contests/practice2/submissions/38270988

use algorithms::data_structure::lazy_segment_tree::LazySegmentTree;
use proconio::input;

fn main() {
    input! { n: usize, q: usize };

    let mut a = Vec::new();

    a.push(i32::MIN);
    for _ in 0..n {
        input! { _a: i32 };
        a.push(_a);
    }
    a.push(i32::MAX);
    a.push(i32::MAX);

    let mut max_segment_tree = LazySegmentTree::from(
        &a,
        || i32::MIN,
        || i32::MIN,
        i32::max,
        |_, b, _, _| b,
        |a, b| if b == i32::MIN { a } else { b },
    );

    for _ in 0..q {
        input! { t: i32 };

        if t == 1 {
            input! { x: usize, v: i32 };
            max_segment_tree.update(x, x + 1, v)
        } else if t == 2 {
            input! { l: usize, r: usize };
            println!("{}", max_segment_tree.get(l, r + 1))
        } else {
            input! { x: usize, v: i32 };

            let mut left = x;
            let mut right = n + 2;

            while right - left > 1 {
                let mid = (left + right) / 2;
                if max_segment_tree.get(left, mid) >= v {
                    right = mid;
                } else {
                    left = mid;
                }
            }
            println!("{}", left);
        }
    }
}
