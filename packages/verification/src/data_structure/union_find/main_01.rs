// problem: https://atcoder.jp/contests/abl/tasks/abl_c
// answer: https://atcoder.jp/contests/abl/submissions/38290086

use algorithms::data_structure::union_find::UnionFind;
use proconio::input;

fn main() {
    input! { n: usize, m: usize };

    let mut uf = UnionFind::new(n + 1);
    let mut ans = n - 1;

    for _ in 0..m {
        input! { a: usize, b: usize };
        if uf.is_same(a, b) {
            continue;
        }
        uf.merge(a, b);
        ans -= 1;
    }

    println!("{}", ans);
}
