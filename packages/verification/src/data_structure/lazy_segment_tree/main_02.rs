// problem: https://atcoder.jp/contests/practice2/tasks/practice2_l
// answer: https://atcoder.jp/contests/practice2/submissions/38271440

use algorithms::data_structure::lazy_segment_tree::LazySegmentTree;
use proconio::input;
use std::fmt::Display;

#[derive(Clone, Copy)]
struct Data {
    inversion_number: usize,
    black: usize,
    white: usize,
}

impl Display for Data {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

fn main() {
    input! { n: usize, q: usize };

    let mut data = Vec::new();

    for _ in 0..n {
        input! { a: usize };
        data.push(Data {
            inversion_number: 0,
            black: a,
            white: 1 - a,
        });
    }

    let mut tree = LazySegmentTree::from(
        &data,
        || Data {
            inversion_number: 0,
            black: 0,
            white: 0,
        },
        || false,
        |a, b| Data {
            inversion_number: a.inversion_number + b.inversion_number + a.black * b.white,
            black: a.black + b.black,
            white: a.white + b.white,
        },
        |d, flip, _, _| {
            if flip {
                Data {
                    inversion_number: d.black * d.white - d.inversion_number,
                    black: d.white,
                    white: d.black,
                }
            } else {
                d
            }
        },
        |a, b| a ^ b,
    );

    for _ in 0..q {
        input! { t: usize, l: usize, r: usize };

        if t == 1 {
            tree.update(l - 1, r, true);
        } else {
            println!("{}", tree.get(l - 1, r).inversion_number);
        }
    }
}
