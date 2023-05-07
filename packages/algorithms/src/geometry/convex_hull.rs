// Andrew's monotone chain convex hull algorithm

use crate::geometry::point2::Point2;
use num_traits::Num;

pub trait ConvecHull<Data: Num> {
    fn convex_hull(self) -> Vec<(usize, Point2<Data>)>;
}

impl<Data: Num> ConvecHull<Data> for Vec<Point2<Data>>
where
    Data: Clone + Copy + PartialOrd + Ord,
{
    fn convex_hull(self) -> Vec<(usize, Point2<Data>)> {
        let mut points: Vec<(usize, Point2<Data>)> = self
            .iter()
            .enumerate()
            .map(|(index, p)| (index, *p))
            .collect();

        points.sort_by(|a, b| {
            if a.1 .0 != b.1 .0 {
                a.1 .0.cmp(&b.1 .0)
            } else {
                a.1 .1.cmp(&b.1 .1)
            }
        });

        let mut result: Vec<(usize, Point2<Data>)> = Vec::new();

        for (index, p) in points.iter() {
            if result.len() > 0 && result[result.len() - 1].1 == *p {
                continue;
            }

            if result.len() < 2 {
                result.push((*index, *p));
                continue;
            }

            loop {
                if result.len() < 2 {
                    break;
                }

                let a = result[result.len() - 1].1;
                let b = result[result.len() - 2].1;
                if (b - a).det(*p - a) <= Data::zero() {
                    break;
                }

                result.pop();
            }
            result.push((*index, *p));
        }

        let upper_len = result.len();

        for (index, p) in points.iter().rev() {
            if result[result.len() - 1].1 == *p {
                continue;
            }

            loop {
                if result.len() <= upper_len {
                    break;
                }

                let a = result[result.len() - 1].1;
                let b = result[result.len() - 2].1;
                if (b - a).det(*p - a) <= Data::zero() {
                    break;
                }

                result.pop();
            }
            result.push((*index, *p));
        }

        result
    }
}
