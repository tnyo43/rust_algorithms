// problem: https://atcoder.jp/contests/abc206/tasks/abc206_d
// answer: https://atcoder.jp/contests/abc206/submissions/38290082

use algorithms::data_structure::union_find::UnionFind;
use proconio::input;

fn main() {
    input! { n: usize };

    let mut a = Vec::new();
    for _ in 0..n {
        input! { _a: usize };
        a.push(_a);
    }

    let mut uf = UnionFind::new(222_222);
    let mut ans = 0;

    for i in 0..n {
        if uf.is_same(a[i], a[n - i - 1]) {
            continue;
        } else {
            uf.merge(a[i], a[n - i - 1]);
            ans += 1;
        }
    }

    println!("{}", ans);
}
