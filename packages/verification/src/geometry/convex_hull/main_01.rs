// problem: https://atcoder.jp/contests/agc021/tasks/agc021_b
// answer: https://atcoder.jp/contests/agc021/submissions/41226720

use std::f64::consts::PI;

use algorithms::geometry::{convex_hull::ConvexHull, point2::Point2};
use proconio::input;

fn main() {
    input! { n: usize };

    let mut points = Vec::new();

    for _ in 0..n {
        input! { x: i64, y: i64 };
        points.push(Point2::new(x, y));
    }

    let mut convex_hull = points.convex_hull();
    convex_hull.pop();

    let mut answer = vec![0.0; n];

    for i in 0..convex_hull.len() {
        let index = convex_hull[(i + 1) % convex_hull.len()].0;
        let left = convex_hull[(i + 1) % convex_hull.len()].1 - convex_hull[i].1;
        let right =
            convex_hull[(i + 2) % convex_hull.len()].1 - convex_hull[(i + 1) % convex_hull.len()].1;

        if left.det(right) == 0 {
            if (left.arg() - right.arg()).abs() < 0.1 {
                answer[index] = 0.0;
            } else {
                answer[index] = 0.5;
            }
            continue;
        }

        let mut angle = right.arg() - left.arg() + 2.0 * PI;
        while angle >= 2.0 * PI {
            angle -= 2.0 * PI;
        }

        answer[index] = angle / (2.0 * PI);
    }

    for a in answer {
        println!("{:.8}", a);
    }
}
