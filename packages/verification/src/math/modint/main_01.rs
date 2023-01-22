// problem: https://atcoder.jp/contests/abc034/tasks/abc034_c
// result: https://atcoder.jp/contests/abc034/submissions/38234383

use algorithms::math::modint::ModCalc;
use proconio::input;

const MAX: usize = 101_010;
const MOD: usize = 1_000_000_007;
type MCalc = ModCalc<MOD>;

fn main() {
    input! { w: usize, h: usize };

    let mcalc = MCalc::new(MAX);
    println!("{}", mcalc.combination(w + h - 2, w - 1));
}
