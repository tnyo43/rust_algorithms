// problem: https://atcoder.jp/contests/abc286/tasks/abc286_h
// answer: https://atcoder.jp/contests/abc286/submissions/42187496

use algorithms::geometry::{convex_hull::ConvexHull, point2::Point2};

use proconio::input;

fn main() {
    input! { n: usize };

    let mut points = Vec::new();

    for _ in 0..n + 2 {
        input! { x: i64, y: i64 };
        points.push(Point2::new(x, y));
    }

    let s = points[points.len() - 2];
    let t = points[points.len() - 1];

    let mut convex_hull = points.convex_hull();
    convex_hull.pop();

    let len = convex_hull.len();
    let mut around = 0.0;
    let mut route = 0.0;
    let mut checking_route = false;

    for i in 0..len {
        around += convex_hull[i].1.distance(convex_hull[(i + 1) % len].1)
    }

    for i in 0..(len * 2) {
        let p = convex_hull[i % len].1;
        let next = convex_hull[(i + 1) % len].1;

        if p == t && checking_route {
            checking_route = false;
            break;
        }

        if p == s {
            checking_route = true;
        }

        if !checking_route {
            continue;
        }

        route += p.distance(next);
    }

    let answer = if route == 0.0 || checking_route {
        s.distance(t)
    } else {
        f64::min(route, around - route)
    };

    println!("{:.8}", answer);
}
