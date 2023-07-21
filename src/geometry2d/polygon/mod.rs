use std::fmt::Debug;
use std::marker::PhantomData;
use std::slice::Iter;

use crate::geometry2d::line::{HitSide, Line2d, ReferenceLine2d};
use crate::geometry2d::polygon::cut::{PointPolygonRelationship, PointRange, PointRangeIterator};
use crate::geometry2d::triangle::static_triangle::StaticTriangle2d;
use crate::geometry2d::triangle::TrianglePointIterator;
use crate::prelude::{BoundingBox, Point2d};

pub trait Polygon2d<P: Point2d>: Sized + Clone + PartialEq + Debug {
    type PointIter<'a>: Iterator<Item = &'a P> + Clone
    where
        Self: 'a,
        P: 'a;
    fn points(&self) -> Self::PointIter<'_>;
    fn point_count(&self) -> usize;
    fn to_any_polygon(self) -> AnyPolygon<P>;
    fn points_of_range(&self, range: &PointRange) -> PolygonRangeIterator<Self, P> {
        PolygonRangeIterator {
            phantom: Default::default(),
            polygon: self,
            idx_iterator: range.iter(),
        }
    }
    fn get_point(&self, idx: usize) -> Option<&P>;
    fn lines(&self) -> PolygonLineIterator<Self::PointIter<'_>, P> {
        PolygonLineIterator::new(self.points())
    }
    fn point_position(&self, p: &P) -> PointPolygonRelationship {
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

    fn bbox(&self) -> BoundingBox {
        let mut ret: BoundingBox = Default::default();
        for p in self.points() {
            ret += p.coordinates();
        }
        ret
    }
}

pub struct PolygonRangeIterator<'a, Poly: Polygon2d<Pt>, Pt: Point2d> {
    phantom: PhantomData<Pt>,
    polygon: &'a Poly,
    idx_iterator: PointRangeIterator,
}

impl<'a, Poly: Polygon2d<Pt>, Pt: Point2d + 'a> Iterator for PolygonRangeIterator<'a, Poly, Pt> {
    type Item = &'a Pt;

    fn next(&mut self) -> Option<Self::Item> {
        self.idx_iterator
            .next()
            .and_then(|idx| self.polygon.get_point(idx))
    }
}

#[derive(Clone)]
pub enum PolygonLineIterator<'a, I: Iterator<Item = &'a Pt> + Clone, Pt: Point2d> {
    Found {
        iterator: I,
        first_point: &'a Pt,
        last_point: &'a Pt,
        done: bool,
    },
    Empty,
}

impl<'a, I: Iterator<Item = &'a Pt> + Clone, Pt: Point2d> Iterator
    for PolygonLineIterator<'a, I, Pt>
{
    type Item = ReferenceLine2d<'a, Pt>;

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
                    let next_line = ReferenceLine2d::<'a, Pt>::new(last_point, next_point);
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

impl<'a, I: Iterator<Item = &'a Pt> + Clone, Pt: Point2d> PolygonLineIterator<'a, I, Pt> {
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

impl<Pt: Point2d> Polygon2d<Pt> for Vec<Pt> {
    type PointIter<'a>    = Iter<'a, Pt> where Self: 'a;

    fn points(&self) -> Self::PointIter<'_> {
        self.iter()
    }

    fn point_count(&self) -> usize {
        self.len()
    }

    fn to_any_polygon(self) -> AnyPolygon<Pt> {
        AnyPolygon::VecPointPolygon(self)
    }

    fn get_point(&self, idx: usize) -> Option<&Pt> {
        self.get(idx)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnyPolygon<P: Point2d> {
    VecPointPolygon(Vec<P>),
    StaticTrianglePolygon(StaticTriangle2d<P>),
}
#[derive(Debug, Clone)]
pub enum AnyPolygonPointIter<'a, Pt: Point2d> {
    VecPointPolygon(Iter<'a, Pt>),
    StaticTrianglePolygon(TrianglePointIterator<'a, StaticTriangle2d<Pt>, Pt>),
}

impl<'a, Pt: Point2d> Iterator for AnyPolygonPointIter<'a, Pt> {
    type Item = &'a Pt;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            AnyPolygonPointIter::VecPointPolygon(i) => i.next(),
            AnyPolygonPointIter::StaticTrianglePolygon(i) => i.next(),
        }
    }
}

impl<Pt: Point2d> Polygon2d<Pt> for AnyPolygon<Pt> {
    type PointIter<'a>    = AnyPolygonPointIter<'a, Pt> where      Self: 'a,;

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

    fn to_any_polygon(self) -> AnyPolygon<Pt> {
        self
    }

    fn get_point(&self, idx: usize) -> Option<&Pt> {
        match self {
            AnyPolygon::VecPointPolygon(p) => p.get_point(idx),
            AnyPolygon::StaticTrianglePolygon(p) => p.get_point(idx),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::geometry2d::point::StaticPoint2d;
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
        let polygon: Vec<StaticPoint2d> = vec![
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
