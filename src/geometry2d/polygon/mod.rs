use std::slice::Iter;

#[cfg(test)]
use svg::node::element::path::Data;

use crate::geometry2d::line::{HitSide, Line2d, ReferenceLine2d};
#[cfg(test)]
use crate::geometry2d::point::BoundingBoxSvg;
use crate::geometry2d::point::Point2d;
use crate::geometry2d::polygon::cut::{PointPolygonRelationship, PointRange, PointRangeIterator};
use crate::geometry2d::triangle::{StaticTriangle2d, TrianglePointIterator};

pub trait Polygon2d: Sized + Clone {
    type PointIter<'a>: Iterator<Item = &'a Point2d> + Clone
    where
        Self: 'a;
    fn points(&self) -> Self::PointIter<'_>;
    fn point_count(&self) -> usize;
    fn to_any_polygon(self) -> AnyPolygon;
    fn points_of_range(&self, range: &PointRange) -> PolygonRangeIterator<Self> {
        PolygonRangeIterator {
            polygon: self,
            idx_iterator: range.iter(),
        }
    }
    fn get_point(&self, idx: usize) -> Option<&Point2d>;
    fn lines(&self) -> PolygonLineIterator<Self::PointIter<'_>> {
        PolygonLineIterator::new(self.points())
    }
    fn point_position(&self, p: &Point2d) -> PointPolygonRelationship {
        let mut right_count = 0;
        for line in self.lines() {
            match line.y_cross_side(p) {
                HitSide::None => {}
                HitSide::OnLine => {
                    return PointPolygonRelationship::OnEdge;
                }
                HitSide::Left => {}
                HitSide::Right => {
                    right_count += 1;
                }

                HitSide::LeftTop => {}
                HitSide::LeftBottom => {}
                HitSide::RightTop => {
                    right_count += 1;
                }
                HitSide::RightBottom => {}
            }
        }
        if right_count % 2 == 0 {
            PointPolygonRelationship::Outside
        } else {
            PointPolygonRelationship::Inside
        }
    }
    #[cfg(test)]
    fn plot(&self, bbox: &BoundingBoxSvg) -> Option<Data> {
        let mut iter = self.points();
        if let Some(start_pt) = iter.next() {
            let mut data = Data::new().move_to(bbox.apply(start_pt));
            for next_pt in iter {
                data = data.line_to(bbox.apply(next_pt));
            }
            Some(data.close())
        } else {
            None
        }
    }
    #[cfg(test)]
    fn bbox(&self) -> BoundingBoxSvg {
        let mut ret: BoundingBoxSvg = Default::default();
        for p in self.points() {
            ret += *p;
        }
        ret
    }
}

pub struct PolygonRangeIterator<'a, P: Polygon2d> {
    polygon: &'a P,
    idx_iterator: PointRangeIterator,
}

impl<'a, P: Polygon2d> Iterator for PolygonRangeIterator<'a, P> {
    type Item = &'a Point2d;

    fn next(&mut self) -> Option<Self::Item> {
        self.idx_iterator
            .next()
            .and_then(|idx| self.polygon.get_point(idx))
    }
}

#[derive(Clone)]
pub enum PolygonLineIterator<'a, I: Iterator<Item = &'a Point2d> + Clone> {
    Found {
        iterator: I,
        first_point: &'a Point2d,
        last_point: &'a Point2d,
        done: bool,
    },
    Empty,
}

impl<'a, I: Iterator<Item = &'a Point2d> + Clone> Iterator for PolygonLineIterator<'a, I> {
    type Item = ReferenceLine2d<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            PolygonLineIterator::Found {
                iterator,
                first_point,
                last_point,
                done,
            } => {
                if *done {
                    None
                } else if let Some(next_point) = iterator.next() {
                    let next_line = ReferenceLine2d::new(last_point, next_point);
                    *last_point = next_point;
                    Some(next_line)
                } else {
                    *done = true;
                    Some(ReferenceLine2d::new(last_point, first_point))
                }
            }
            PolygonLineIterator::Empty => None,
        }
    }
}

impl<'a, I: Iterator<Item = &'a Point2d> + Clone> PolygonLineIterator<'a, I> {
    fn new(mut iterator: I) -> Self {
        if let Some(first_point) = iterator.next() {
            PolygonLineIterator::Found {
                iterator,
                first_point,
                last_point: first_point,
                done: false,
            }
        } else {
            PolygonLineIterator::Empty
        }
    }
}

pub mod cut;

impl Polygon2d for Vec<Point2d> {
    type PointIter<'a>

    = Iter<'a, Point2d> where    Self: 'a;

    fn points(&self) -> Self::PointIter<'_> {
        self.iter()
    }

    fn point_count(&self) -> usize {
        self.len()
    }

    fn to_any_polygon(self) -> AnyPolygon {
        AnyPolygon::VecPointPolygon(self)
    }

    fn get_point(&self, idx: usize) -> Option<&Point2d> {
        self.get(idx)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnyPolygon {
    VecPointPolygon(Vec<Point2d>),
    StaticTrianglePolygon(StaticTriangle2d),
}
#[derive(Debug, Clone)]
pub enum AnyPolygonPointIter<'a> {
    VecPointPolygon(Iter<'a, Point2d>),
    StaticTrianglePolygon(TrianglePointIterator<'a, StaticTriangle2d>),
}

impl<'a> Iterator for AnyPolygonPointIter<'a> {
    type Item = &'a Point2d;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            AnyPolygonPointIter::VecPointPolygon(i) => i.next(),
            AnyPolygonPointIter::StaticTrianglePolygon(i) => i.next(),
        }
    }
}

impl Polygon2d for AnyPolygon {
    type PointIter<'a>    = AnyPolygonPointIter<'a> where      Self: 'a,;

    fn points(&self) -> Self::PointIter<'_> {
        match self {
            AnyPolygon::VecPointPolygon(p) => AnyPolygonPointIter::VecPointPolygon(p.points()),
            AnyPolygon::StaticTrianglePolygon(p) => {
                AnyPolygonPointIter::StaticTrianglePolygon(p.points())
            }
        }
    }

    fn point_count(&self) -> usize {
        match self {
            AnyPolygon::VecPointPolygon(p) => p.point_count(),
            AnyPolygon::StaticTrianglePolygon(p) => p.point_count(),
        }
    }

    fn to_any_polygon(self) -> AnyPolygon {
        self
    }

    fn get_point(&self, idx: usize) -> Option<&Point2d> {
        match self {
            AnyPolygon::VecPointPolygon(p) => p.get_point(idx),
            AnyPolygon::StaticTrianglePolygon(p) => p.get_point(idx),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::geometry2d::point::Point2d;
    use crate::geometry2d::polygon::cut::{PointPolygonRelationship, PointRange};
    use crate::geometry2d::polygon::Polygon2d;

    #[test]
    fn test_point_range() {
        let mut empty_iterator = PointRange::None.iter();
        assert_eq!(None, empty_iterator.next());

        let mut strait_iterator = PointRange::SingleRange {
            first_idx: 1,
            last_idx: 2,
        }
        .iter();
        assert_eq!(Some(1), strait_iterator.next());
        assert_eq!(Some(2), strait_iterator.next());
        assert_eq!(None, strait_iterator.next());

        let mut wrap_iterator = PointRange::WarpAround {
            first_idx: 2,
            last_idx: 1,
            point_count: 3,
        }
        .iter();
        assert_eq!(Some(2), wrap_iterator.next());
        assert_eq!(Some(0), wrap_iterator.next());
        assert_eq!(Some(1), wrap_iterator.next());
        assert_eq!(None, wrap_iterator.next());
    }
    #[test]
    fn test_point_range_reverse() {
        let mut empty_iterator = PointRange::None.reverse_iter();
        assert_eq!(None, empty_iterator.next());

        let mut strait_iterator = PointRange::SingleRange {
            first_idx: 1,
            last_idx: 2,
        }
        .reverse_iter();
        assert_eq!(Some(2), strait_iterator.next());
        assert_eq!(Some(1), strait_iterator.next());
        assert_eq!(None, strait_iterator.next());

        let mut wrap_iterator = PointRange::WarpAround {
            first_idx: 2,
            last_idx: 1,
            point_count: 3,
        }
        .reverse_iter();
        assert_eq!(Some(1), wrap_iterator.next());
        assert_eq!(Some(0), wrap_iterator.next());
        assert_eq!(Some(2), wrap_iterator.next());
        assert_eq!(None, wrap_iterator.next());
    }

    #[test]
    fn test_polygon_point() {
        let polygon: Vec<Point2d> = vec![
            (0.0, 0.0).into(),
            (1.0, 0.0).into(),
            (1.0, 1.0).into(),
            (0.0, 1.0).into(),
        ];
        assert_eq!(
            PointPolygonRelationship::Inside,
            polygon.point_position(&(0.5, 0.5).into())
        );
        assert_eq!(
            PointPolygonRelationship::Outside,
            polygon.point_position(&(-0.5, 0.5).into())
        );
        assert_eq!(
            PointPolygonRelationship::Outside,
            polygon.point_position(&(1.5, 0.5).into())
        );
        assert_eq!(
            PointPolygonRelationship::Outside,
            polygon.point_position(&(0.5, -0.5).into())
        );
        assert_eq!(
            PointPolygonRelationship::Outside,
            polygon.point_position(&(0.5, 1.5).into())
        );
        assert_eq!(
            PointPolygonRelationship::Outside,
            polygon.point_position(&(-0.5, 0.0).into())
        );
        assert_eq!(
            PointPolygonRelationship::Outside,
            polygon.point_position(&(1.5, 0.0).into())
        );

        assert_eq!(
            PointPolygonRelationship::OnEdge,
            polygon.point_position(&(0.5, 0.0).into())
        );
        assert_eq!(
            PointPolygonRelationship::OnEdge,
            polygon.point_position(&(0.0, 0.5).into())
        );
        assert_eq!(
            PointPolygonRelationship::OnEdge,
            polygon.point_position(&(0.0, 0.0).into())
        );
        assert_eq!(
            PointPolygonRelationship::OnEdge,
            polygon.point_position(&(1.0, 1.0).into())
        );
    }
}
