use std::fmt::{Debug, Formatter};

use log::error;
use num_traits::{One, Zero};
use ordered_float::OrderedFloat;
use triangulate::Vertex;

use crate::geometry2d::line::{Line2d, LineIntersection, ReferenceLine2d, SideOfLine};
use crate::geometry2d::point::Point2d;
use crate::geometry2d::polygon::{
    AnyPolygon, CutSegment, LineCutIdx, PointRange, Polygon2d, PolygonPath,
};
use crate::primitives::Number;

pub trait Triangle2d: Sized + Polygon2d {
    fn p1(&self) -> &Point2d;
    fn p2(&self) -> &Point2d;
    fn p3(&self) -> &Point2d;
    fn l1(&self) -> (&Point2d, &Point2d) {
        (self.p1(), self.p2())
    }
    fn l2(&self) -> (&Point2d, &Point2d) {
        (self.p2(), self.p3())
    }
    fn l3(&self) -> (&Point2d, &Point2d) {
        (self.p3(), self.p1())
    }
    /*
    fn points(&self) -> TrianglePointIterator<'_, Self> {
        TrianglePointIterator {
            triangle: self,
            state: Default::default(),
        }
    }*/
    /*
    fn lines(&self) -> TriangleLineIterator<'_, Self> {
        TriangleLineIterator {
            triangle: self,
            state: Default::default(),
        }
    }

     */
    fn area(&self) -> Number {
        let p1 = self.p1();
        let p2 = self.p2();
        let p3 = self.p3();
        (p1.x() * (p2.y() - p3.y()) + p2.x() * (p3.y() - p1.y()) + p3.x() * (p1.y() - p2.y())) / 2.0
    }
    fn quadrant_pattern(&self, p: &Point2d) -> [SideOfLine; 3] {
        [
            self.l1().side_of_pt(p),
            self.l2().side_of_pt(p),
            self.l3().side_of_pt(p),
        ]
    }

    fn find_cutting_edge<L: Line2d>(&self, line: &L) -> Option<(usize, Number, Number)> {
        self.lines()
            .enumerate()
            .filter_map::<(usize, Number, Number), _>(|(idx, tr_line)| {
                let l: ReferenceLine2d = tr_line;
                let x = l.intersect(line);
                <LineIntersection as Into<Option<(Number, Number)>>>::into(x)
                    .map(|(triangle_pos, polygon_pos)| (idx, triangle_pos, polygon_pos))
            })
            .max_by_key(|(_, pos, _)| (*pos).min(Number::one() - *pos))
    }
    fn find_cutting_edges<L: Line2d>(&self, line: &L) -> Vec<(usize, Number, Number)> {
        let mut edges: Vec<_> = self
            .lines()
            .enumerate()
            .filter_map::<(usize, Number, Number), _>(|(idx, tr_line)| {
                <LineIntersection as Into<Option<(Number, Number)>>>::into(tr_line.intersect(line))
                    .filter(|(t, _)| *t > Number::zero() && *t < Number::one())
                    .map(|(triangle_pos, polygon_pos)| (idx, triangle_pos, polygon_pos))
            })
            .collect();
        edges.sort_by_key(|(_, pos, _)| -(*pos).min(Number::one() - *pos));
        edges
    }

    fn cut<P: Polygon2d + Debug>(&self, cut_polygon: &P) -> PolygonPath {
        assert!(self.area() > OrderedFloat::from(0.0));
        let mut poly_point_iter = cut_polygon.points().enumerate();
        if let Some((_, first_poly_pt)) = poly_point_iter.next() {
            let mut last_poly_pt = first_poly_pt;
            let mut last_pattern = self.quadrant_pattern(&last_poly_pt);
            let mut current_polygon = if pt_is_inside(&last_pattern) {
                Some(vec![(0, *first_poly_pt)])
            } else {
                None
            };
            let mut found_polygons = Vec::new();
            let mut last_idx = 0;

            for (idx, next_pt) in poly_point_iter.chain(Some(first_poly_pt).into_iter().enumerate())
            {
                let next_pattern = self.quadrant_pattern(&next_pt);
                if last_pattern == next_pattern {
                    // same quadrant as before
                    if let Some(polygon) = current_polygon.as_mut() {
                        // was inside, keep inside
                        polygon.push((idx, *next_pt));
                    }
                } else {
                    if pt_is_inside(&next_pattern) {
                        // entered triangle
                        current_polygon.replace(vec![(last_idx, *last_poly_pt), (idx, *next_pt)]);
                    } else if let Some(mut polygon) = current_polygon.take() {
                        // last was inside, end polygon
                        polygon.push((idx, *next_pt));
                        found_polygons.push(polygon);
                    } else if !(next_pattern
                        .iter()
                        .zip(last_pattern.iter())
                        .any(|(next, last)| {
                            next == &SideOfLine::Right && last == &SideOfLine::Right
                        }))
                    {
                        // could have crossed 2 lines -> keep candidate
                        found_polygons.push(vec![(last_idx, *last_poly_pt), (idx, *next_pt)]);
                    }
                    last_pattern = next_pattern;
                }
                last_poly_pt = next_pt;
                last_idx = idx;
            }
            if let Some(mut last_poly) = current_polygon {
                if let Some(first_poly) = found_polygons.first_mut() {
                    last_poly.pop();
                    first_poly.splice(0..0, last_poly);
                } else {
                    return PolygonPath::Enclosed;
                }
            }
            if found_polygons.is_empty() {
                PolygonPath::None
            } else {
                // find exact cut points
                let segments: Vec<CutSegment> = found_polygons
                    .into_iter()
                    .filter_map(|polygon| {
                        let point_count = polygon.len();
                        if let (
                            Some((first_idx_begin, first_pt_begin)),
                            Some((first_idx_end, first_pt_end)),
                            Some((last_idx_begin, last_pt_begin)),
                            Some((_, last_pt_end)),
                        ) = (
                            polygon.get(0),
                            polygon.get(1),
                            polygon.get(point_count - 2),
                            polygon.get(point_count - 1),
                        ) {
                            if first_idx_begin == last_idx_begin {
                                // only one segment
                                let line = ReferenceLine2d::new(first_pt_begin, first_pt_end);
                                let mut line_cut = self.find_cutting_edges(&line);
                                line_cut.truncate(2);
                                line_cut.sort_by_key(|(_, _, poly_pos)| *poly_pos);
                                let mut iter = line_cut.iter();
                                let first_cut = iter.next();
                                let second_cut = iter.next();
                                if let (
                                    Some((first_idx, _, first_pos)),
                                    Some((second_idx, _, second_pos)),
                                ) = (first_cut, second_cut)
                                {
                                    Some(CutSegment::new(
                                        PointRange::None,
                                        LineCutIdx::new(*first_idx_begin, *first_idx, *first_pos),
                                        LineCutIdx::new(*last_idx_begin, *second_idx, *second_pos),
                                    ))
                                } else {
                                    None
                                }
                            } else {
                                let start_line = ReferenceLine2d::new(first_pt_begin, first_pt_end);
                                let start_line_cut = self.find_cutting_edge(&start_line);
                                let end_line = ReferenceLine2d::new(last_pt_begin, last_pt_end);
                                let end_line_cut = self.find_cutting_edge(&end_line);
                                if let (
                                    Some((start_line_idx, _, start_polygon_pos)),
                                    Some((end_line_idx, _, end_polygon_pos)),
                                ) = (start_line_cut, end_line_cut)
                                {
                                    let range = if *first_idx_end < *last_idx_begin {
                                        PointRange::SingleRange {
                                            first_idx: *first_idx_end,
                                            last_idx: *last_idx_begin,
                                        }
                                    } else {
                                        PointRange::WarpAround {
                                            first_idx: *first_idx_end,
                                            last_idx: *last_idx_begin,
                                            point_count,
                                        }
                                    };
                                    Some(CutSegment::new(
                                        range,
                                        LineCutIdx::new(
                                            *first_idx_begin,
                                            start_line_idx,
                                            start_polygon_pos,
                                        ),
                                        LineCutIdx::new(
                                            *last_idx_begin,
                                            end_line_idx,
                                            end_polygon_pos,
                                        ),
                                    ))
                                } else {
                                    error!("Cut Position not found at {polygon:?}");
                                    None
                                }
                            }
                        } else {
                            error!("Invalid cut segment: {polygon:?}");
                            None // invalid segment (should not happen)
                        }
                    })
                    .collect();
                if segments.is_empty() {
                    PolygonPath::None
                } else {
                    PolygonPath::CutSegments(segments)
                }
            }
        } else {
            error!("Invalid cut polygon: {cut_polygon:?}");
            PolygonPath::None
        }
    }
}
#[derive(Debug)]
pub struct TrianglePointIterator<'a, T: Triangle2d> {
    triangle: &'a T,
    state: TriangleIteratorState,
}
#[derive(Default, Debug)]
enum TriangleIteratorState {
    #[default]
    P1,
    P2,
    P3,
    BeyondLast,
}
impl TriangleIteratorState {
    fn next_state(&self) -> Self {
        match self {
            TriangleIteratorState::P1 => TriangleIteratorState::P2,
            TriangleIteratorState::P2 => TriangleIteratorState::P3,
            TriangleIteratorState::P3 => TriangleIteratorState::BeyondLast,
            TriangleIteratorState::BeyondLast => TriangleIteratorState::BeyondLast,
        }
    }
    fn remaining_count(&self) -> usize {
        match self {
            TriangleIteratorState::P1 => 3,
            TriangleIteratorState::P2 => 2,
            TriangleIteratorState::P3 => 1,
            TriangleIteratorState::BeyondLast => 0,
        }
    }
}
impl<'a, T: Triangle2d> Iterator for TrianglePointIterator<'a, T> {
    type Item = &'a Point2d;

    fn next(&mut self) -> Option<Self::Item> {
        let next_value = match self.state {
            TriangleIteratorState::P1 => Some(self.triangle.p1()),
            TriangleIteratorState::P2 => Some(self.triangle.p2()),
            TriangleIteratorState::P3 => Some(self.triangle.p3()),
            TriangleIteratorState::BeyondLast => None,
        };
        self.state = self.state.next_state();
        next_value
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining_count = self.state.remaining_count();
        (remaining_count, Some(remaining_count))
    }
}
impl<T: Triangle2d> ExactSizeIterator for TrianglePointIterator<'_, T> {
    fn len(&self) -> usize {
        self.state.remaining_count()
    }
}

#[derive(Debug)]
pub struct TriangleLineIterator<'a, T: Triangle2d> {
    triangle: &'a T,
    state: TriangleIteratorState,
}

pub struct TriangleLine<'a, T: Triangle2d> {
    triangle: &'a T,
    side: TriangleSide,
}

impl<'a, T: Triangle2d> Line2d for TriangleLine<'a, T> {
    fn p1(&self) -> &Point2d {
        match self.side {
            TriangleSide::S1 => self.triangle.p1(),
            TriangleSide::S2 => self.triangle.p2(),
            TriangleSide::S3 => self.triangle.p3(),
        }
    }

    fn p2(&self) -> &Point2d {
        match self.side {
            TriangleSide::S1 => self.triangle.p2(),
            TriangleSide::S2 => self.triangle.p3(),
            TriangleSide::S3 => self.triangle.p1(),
        }
    }
}
impl<'a, T: Triangle2d> Debug for TriangleLine<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({:?})-({:?})", self.p1(), self.p2()))
    }
}

#[derive(Debug)]
enum TriangleSide {
    S1,
    S2,
    S3,
}

impl<'a, T: Triangle2d> Iterator for TriangleLineIterator<'a, T> {
    type Item = TriangleLine<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_value = match self.state {
            TriangleIteratorState::P1 => Some(TriangleLine {
                triangle: self.triangle,
                side: TriangleSide::S1,
            }),
            TriangleIteratorState::P2 => Some(TriangleLine {
                triangle: self.triangle,
                side: TriangleSide::S2,
            }),
            TriangleIteratorState::P3 => Some(TriangleLine {
                triangle: self.triangle,
                side: TriangleSide::S3,
            }),
            TriangleIteratorState::BeyondLast => None,
        };
        self.state = self.state.next_state();
        next_value
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining_count = self.state.remaining_count();
        (remaining_count, Some(remaining_count))
    }
}
fn pt_is_inside(pattern: &[SideOfLine; 3]) -> bool {
    pattern.iter().all(|s| *s == SideOfLine::Left)
}
fn pt_is_outside(pattern: &[SideOfLine; 3]) -> bool {
    pattern.iter().any(|s| *s == SideOfLine::Right)
}
#[derive(Clone, PartialEq)]
pub struct StaticTriangle2d {
    p1: Point2d,
    p2: Point2d,
    p3: Point2d,
}

impl Debug for StaticTriangle2d {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "T2d({:?},{:?},{:?})",
            self.p1, self.p2, self.p3
        ))
    }
}
impl StaticTriangle2d {
    pub fn new(p1: Point2d, p2: Point2d, p3: Point2d) -> Self {
        Self { p1, p2, p3 }
    }
}

impl Polygon2d for StaticTriangle2d {
    type PointIter<'a> = TrianglePointIterator<'a, StaticTriangle2d>;

    fn points(&self) -> Self::PointIter<'_> {
        TrianglePointIterator {
            triangle: self,
            state: Default::default(),
        }
    }

    fn point_count(&self) -> usize {
        3
    }

    fn to_any_polygon(self) -> AnyPolygon {
        AnyPolygon::StaticTrianglePolygon(self)
    }

    fn get_point(&self, idx: usize) -> Option<&'_ Point2d> {
        match idx {
            0 => Some(&self.p1),
            1 => Some(&self.p2),
            2 => Some(&self.p3),
            _ => None,
        }
    }
}
impl Triangle2d for StaticTriangle2d {
    fn p1(&self) -> &Point2d {
        &self.p1
    }

    fn p2(&self) -> &Point2d {
        &self.p2
    }

    fn p3(&self) -> &Point2d {
        &self.p3
    }
}
