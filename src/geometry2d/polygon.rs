use std::slice::Iter;

#[cfg(test)]
use svg::node::element::path::Data;

use crate::geometry2d::line::ReferenceLine2d;
#[cfg(test)]
use crate::geometry2d::point::BoundingBoxSvg;
use crate::geometry2d::point::Point2d;
use crate::geometry2d::triangle::TrianglePointIterator;
use crate::geometry2d::StaticTriangle2d;
use crate::primitives::Number;

pub trait Polygon2d: Sized + Clone {
    type PointIter<'a>: Iterator<Item = &'a Point2d>
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

pub enum PolygonLineIterator<'a, I: Iterator<Item = &'a Point2d>> {
    Found {
        iterator: I,
        first_point: &'a Point2d,
        last_point: &'a Point2d,
        done: bool,
    },
    Empty,
}

impl<'a, I: Iterator<Item = &'a Point2d>> Iterator for PolygonLineIterator<'a, I> {
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

impl<'a, I: Iterator<Item = &'a Point2d>> PolygonLineIterator<'a, I> {
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
#[derive(Debug)]
pub enum PolygonPath {
    Enclosed,
    CutSegments(Vec<CutSegment>),
    None,
}

#[derive(Debug)]
pub struct CutSegment {
    copy_points: PointRange,
    start_cut: LineCutIdx,
    end_cut: LineCutIdx,
}
#[derive(Debug)]
pub enum PointRange {
    None,
    SingleRange {
        first_idx: usize,
        last_idx: usize,
    },
    WarpAround {
        first_idx: usize,
        last_idx: usize,
        point_count: usize,
    },
}

impl PointRange {
    fn iter(&self) -> PointRangeIterator {
        match self {
            PointRange::None => PointRangeIterator::None,
            PointRange::SingleRange {
                first_idx,
                last_idx,
            } => PointRangeIterator::SingleRange {
                next_value: Some(*first_idx),
                last_idx: *last_idx,
            },
            PointRange::WarpAround {
                first_idx,
                last_idx,
                point_count,
            } => PointRangeIterator::WarpAround {
                last_idx: *last_idx,
                point_count: *point_count,
                next_value: Some(*first_idx),
            },
        }
    }
}

pub enum PointRangeIterator {
    None,
    SingleRange {
        last_idx: usize,
        next_value: Option<usize>,
    },
    WarpAround {
        last_idx: usize,
        point_count: usize,
        next_value: Option<usize>,
    },
}

impl Iterator for PointRangeIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            PointRangeIterator::None => None,
            PointRangeIterator::SingleRange {
                last_idx,
                next_value,
            } => {
                if let Some(value) = next_value.clone() {
                    *next_value = if value < *last_idx {
                        Some(value + 1)
                    } else {
                        None
                    };
                    Some(value)
                } else {
                    None
                }
            }
            PointRangeIterator::WarpAround {
                last_idx,
                point_count,
                next_value,
            } => {
                if let Some(value) = *next_value {
                    *next_value = if value == *last_idx {
                        None
                    } else if value < *point_count - 1 {
                        Some(value + 1)
                    } else {
                        Some(0)
                    };
                    Some(value)
                } else {
                    None
                }
            }
        }
    }
}

impl CutSegment {
    pub fn new(copy_points: PointRange, start_cut: LineCutIdx, end_cut: LineCutIdx) -> Self {
        Self {
            copy_points,
            start_cut,
            end_cut,
        }
    }

    pub fn start_cut(&self) -> &LineCutIdx {
        &self.start_cut
    }
    pub fn end_cut(&self) -> &LineCutIdx {
        &self.end_cut
    }
    pub fn copy_points(&self) -> &PointRange {
        &self.copy_points
    }
}

#[derive(Debug)]
pub struct LineCutIdx {
    start_pt_idx: usize,
    triangle_line_idx: usize,
    cut_pos: Number,
}

impl LineCutIdx {
    pub fn new(start_pt_idx: usize, triangle_line_idx: usize, cut_pos: Number) -> Self {
        Self {
            start_pt_idx,
            triangle_line_idx,
            cut_pos,
        }
    }

    pub fn start_pt_idx(&self) -> usize {
        self.start_pt_idx
    }
    pub fn triangle_line_idx(&self) -> usize {
        self.triangle_line_idx
    }
    pub fn cut_pos(&self) -> Number {
        self.cut_pos
    }
}

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
    use crate::geometry2d::polygon::PointRange;

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
}
