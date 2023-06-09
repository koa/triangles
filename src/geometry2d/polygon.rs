use std::ops::Range;
use std::slice::Iter;

use crate::geometry2d::line::ReferenceLine2d;
use crate::geometry2d::point::Point2d;
use crate::primitives::Number;

pub trait Polygon2d: Sized {
    type PointIter<'a>: Iterator<Item = &'a Point2d>
    where
        Self: 'a;
    fn points(&self) -> Self::PointIter<'_>;
    fn point_count(&self) -> usize;
    fn points_of_range<'a>(&'a self, range: Range<usize>) -> PolygonRangeIterator<'a, Self> {
        let (r1, r2) = if range.start < range.end {
            (range, 0..0)
        } else {
            (range.start..self.point_count(), 0..range.end)
        };
        let next_idx = r1.start;
        PolygonRangeIterator {
            polygon: self,
            r1,
            r2,
            segment: PolygonRangeIteratorSegment::R1,
            next_idx,
        }
    }
    fn get_point(&self, idx: usize) -> Option<&Point2d>;
    fn lines(&self) -> PolygonLineIterator<Self::PointIter<'_>> {
        PolygonLineIterator::new(self.points())
    }
}

pub struct PolygonRangeIterator<'a, P: Polygon2d> {
    polygon: &'a P,
    r1: Range<usize>,
    r2: Range<usize>,
    segment: PolygonRangeIteratorSegment,
    next_idx: usize,
}

enum PolygonRangeIteratorSegment {
    R1,
    R2,
    DONE,
}

impl<'a, P: Polygon2d> Iterator for PolygonRangeIterator<'a, P> {
    type Item = &'a Point2d;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.next_idx;
        match self.segment {
            PolygonRangeIteratorSegment::R1 => {
                if idx >= self.r1.end {
                    self.segment = PolygonRangeIteratorSegment::R2;
                    self.next_idx = self.r2.start;
                    self.next()
                } else {
                    self.next_idx += 1;
                    self.polygon.get_point(idx)
                }
            }
            PolygonRangeIteratorSegment::R2 => {
                if idx >= self.r2.end {
                    self.segment = PolygonRangeIteratorSegment::DONE;
                    None
                } else {
                    self.next_idx += 1;
                    self.polygon.get_point(idx)
                }
            }
            PolygonRangeIteratorSegment::DONE => None,
        }
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
    range: Range<usize>,
    start_cut: LineCutIdx,
    end_cut: LineCutIdx,
}

impl CutSegment {
    pub fn new(range: Range<usize>, start_cut: LineCutIdx, end_cut: LineCutIdx) -> Self {
        Self {
            range,
            start_cut,
            end_cut,
        }
    }

    pub fn range(&self) -> &Range<usize> {
        &self.range
    }
    pub fn start_cut(&self) -> &LineCutIdx {
        &self.start_cut
    }
    pub fn end_cut(&self) -> &LineCutIdx {
        &self.end_cut
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

    fn get_point(&self, idx: usize) -> Option<&Point2d> {
        self.get(idx)
    }
}
