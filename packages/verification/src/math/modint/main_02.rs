// problem: https://atcoder.jp/contests/abc163/tasks/abc163_d
// result: https://atcoder.jp/contests/abc163/submissions/38241292

use algorithms::math::modint::Modint;
use proconio::input;

const MOD: usize = 1_000_000_007;
type M = Modint<MOD>;

fn main() {
    input! { n: usize, k: usize };

    let mut ans = M::new(0);
    for l in k..=n + 1 {
        ans += Modint::new(l) * (Modint::new(n) - Modint::new(l) + Modint::new(1)) + Modint::new(1);
    }

    println!("{}", ans);
}
