use std::{
    fmt::{Debug, Formatter},
    vec,
};

use itertools::Itertools;
use log::error;
use num_traits::{One, Zero};
use ordered_float::OrderedFloat;
use triangulate::{formats, ListFormat, Polygon, PolygonList};

use crate::{
    geometry2d::{
        line::{Line2d, LineIntersection, ReferenceLine2d, SideOfLine},
        point::{Point2d, StaticPoint2d},
        polygon::{
            cut::{CutSegment, LineCutIdx, PointRange, PolygonPath},
            Polygon2d,
        },
        triangle::cut::{walk_shape_recursive, TraceResultPoint},
    },
    prelude::StaticTriangle2d,
    primitives::Number,
};

pub mod static_triangle;

pub trait Triangle2d: Sized + Polygon2d {
    fn p1(&self) -> &StaticPoint2d;
    fn p2(&self) -> &StaticPoint2d;
    fn p3(&self) -> &StaticPoint2d;
    fn l1(&self) -> (&StaticPoint2d, &StaticPoint2d) {
        (self.p1(), self.p2())
    }
    fn l2(&self) -> (&StaticPoint2d, &StaticPoint2d) {
        (self.p2(), self.p3())
    }
    fn l3(&self) -> (&StaticPoint2d, &StaticPoint2d) {
        (self.p3(), self.p1())
    }
    fn point(&self, p: TriangleCornerPoint) -> &StaticPoint2d {
        match p {
            TriangleCornerPoint::P1 => self.p1(),
            TriangleCornerPoint::P2 => self.p2(),
            TriangleCornerPoint::P3 => self.p3(),
        }
    }
    fn side(&self, p: TriangleSide) -> (&StaticPoint2d, &StaticPoint2d) {
        match p {
            TriangleSide::S1 => self.l1(),
            TriangleSide::S2 => self.l2(),
            TriangleSide::S3 => self.l3(),
        }
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
    fn contains_pt(&self, p: &StaticPoint2d) -> bool {
        self.quadrant_pattern(p) == [SideOfLine::Left, SideOfLine::Left, SideOfLine::Left]
    }
    fn quadrant_pattern(&self, p: &StaticPoint2d) -> [SideOfLine; 3] {
        [
            self.l1().side_of_pt(p),
            self.l2().side_of_pt(p),
            self.l3().side_of_pt(p),
        ]
    }
    fn reverse(&self) -> Self;

    fn find_cutting_edge<L: Line2d>(&self, line: &L) -> Option<(TriangleSide, Number, Number)> {
        self.lines()
            .enumerate()
            .filter_map::<(TriangleSide, Number, Number), _>(|(idx, tr_line)| {
                let l: ReferenceLine2d = tr_line;
                let x = l.intersect(line);
                <LineIntersection as Into<Option<(Number, Number)>>>::into(x)
                    .map(|(triangle_pos, polygon_pos)| (idx.into(), triangle_pos, polygon_pos))
            })
            .max_by_key(|(_, pos, _)| (*pos).min(Number::one() - *pos))
    }
    fn find_cutting_edges<L: Line2d>(&self, line: &L) -> Vec<(TriangleSide, Number, Number)> {
        let mut edges: Vec<_> = self
            .lines()
            .enumerate()
            .filter_map::<(TriangleSide, Number, Number), _>(|(idx, tr_line)| {
                <LineIntersection as Into<Option<(Number, Number)>>>::into(tr_line.intersect(line))
                    .filter(|(t, _)| *t > Number::zero() && *t < Number::one())
                    .map(|(triangle_pos, polygon_pos)| (idx.into(), triangle_pos, polygon_pos))
            })
            .collect();
        edges.sort_by_key(|(_, pos, _)| -(*pos).min(Number::one() - *pos));
        edges
    }

    fn cut<P: Polygon2d + Debug>(&self, cut_polygon: &P) -> PolygonPath {
        assert!(self.area() > OrderedFloat::zero());
        let mut poly_point_iter = cut_polygon.points().enumerate();
        if let Some((_, first_poly_pt)) = poly_point_iter.next() {
            let mut last_poly_pt = first_poly_pt;
            let mut last_pattern = self.quadrant_pattern(last_poly_pt);
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
                                    Some((first_idx, first_triangle_pos, first_pos)),
                                    Some((second_idx, second_triangle_pos, second_pos)),
                                ) = (first_cut, second_cut)
                                {
                                    Some(CutSegment::new(
                                        PointRange::None,
                                        LineCutIdx::new(
                                            *first_idx_begin,
                                            *first_idx as TriangleSide,
                                            *first_pos,
                                            *first_triangle_pos,
                                        ),
                                        LineCutIdx::new(
                                            *last_idx_begin,
                                            *second_idx,
                                            *second_pos,
                                            *second_triangle_pos,
                                        ),
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
                                    Some((start_line_idx, start_trigangle_pos, start_polygon_pos)),
                                    Some((end_line_idx, end_triangle_pos, end_polygon_pos)),
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
                                            point_count: cut_polygon.point_count(),
                                        }
                                    };
                                    Some(CutSegment::new(
                                        range,
                                        LineCutIdx::new(
                                            *first_idx_begin,
                                            start_line_idx,
                                            start_polygon_pos,
                                            start_trigangle_pos,
                                        ),
                                        LineCutIdx::new(
                                            *last_idx_begin,
                                            end_line_idx,
                                            end_polygon_pos,
                                            end_triangle_pos,
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
    fn triangulate_cut_polygons<P: Polygon2d + Debug>(
        &self,
        cut_polygon: &P,
        path: &PolygonPath,
    ) -> [Vec<StaticTriangle2d>; 2] {
        match path {
            PolygonPath::Enclosed => {
                let outer_polygon: Vec<StaticPoint2d> = self.points().copied().collect();
                let inner_polygon: Vec<StaticPoint2d> = cut_polygon.points().copied().collect();
                let outer_shape = vec![outer_polygon, inner_polygon];
                let mut outer_triangulated_indices = Vec::<[usize; 2]>::new();
                let outer_triangles = outer_shape
                    .triangulate(
                        formats::IndexedListFormat::new(&mut outer_triangulated_indices)
                            .into_fan_format(),
                    )
                    .expect("Error triangulate outer polygons");
                let mut point_iter = outer_triangles.iter();
                let mut final_outer_triangles = Vec::with_capacity(outer_triangles.len() / 3);
                while let (Some(p1), Some(p2), Some(p3)) =
                    (point_iter.next(), point_iter.next(), point_iter.next())
                {
                    final_outer_triangles.push(StaticTriangle2d::new(
                        outer_shape[p1[0]][p1[1]],
                        outer_shape[p2[0]][p2[1]],
                        outer_shape[p3[0]][p3[1]],
                    ));
                }
                let inner_triangles = if cut_polygon.point_count() == 3 {
                    if let (Some(p1), Some(p2), Some(p3)) = (
                        cut_polygon.get_point(0),
                        cut_polygon.get_point(1),
                        cut_polygon.get_point(2),
                    ) {
                        vec![StaticTriangle2d::new(*p1, *p2, *p3)]
                    } else {
                        panic!("invalid cut polygon")
                    }
                } else {
                    let point_list: Vec<_> = cut_polygon.points().copied().collect();
                    let mut inner_triangulated_indices = Vec::<usize>::new();
                    let inner_triangles = point_list
                        .triangulate(
                            formats::IndexedListFormat::new(&mut inner_triangulated_indices)
                                .into_fan_format(),
                        )
                        .expect("Error triangulate inner polygons");
                    let mut point_iter = inner_triangles.iter();
                    let mut triangles = Vec::with_capacity(inner_triangles.len() / 3);
                    while let (Some(p1), Some(p2), Some(p3)) =
                        (point_iter.next(), point_iter.next(), point_iter.next())
                    {
                        triangles.push(StaticTriangle2d::new(
                            point_list[*p1],
                            point_list[*p2],
                            point_list[*p3],
                        ));
                    }
                    triangles
                };
                [final_outer_triangles, inner_triangles]
            }
            PolygonPath::CutSegments(segments) => walk_shape_recursive(segments).map(|p| {
                Iterator::flat_map(p.iter(), |polygon| {
                    let mut remaining_polygon =
                        Iterator::map(polygon.iter().dedup(), |point| match point {
                            TraceResultPoint::Corner(p) => *self.point(*p),
                            TraceResultPoint::PolygonPoint(p) => {
                                *cut_polygon.get_point(*p).unwrap()
                            }
                            TraceResultPoint::CrossPoint {
                                triangle_side,
                                along_triangle,
                                ..
                            } => self.side(*triangle_side).pt_along(*along_triangle),
                        })
                        .collect::<Vec<_>>();
                    remaining_polygon.dedup();
                    let mut triangulated_indices = Vec::<usize>::new();
                    let x1 = remaining_polygon
                        .triangulate(
                            formats::IndexedListFormat::new(&mut triangulated_indices)
                                .into_fan_format(),
                        )
                        .expect("Error on triangulate");
                    let mut point_iter = x1.iter();
                    let mut triangles = Vec::with_capacity(x1.len() / 3);
                    while let (Some(p1), Some(p2), Some(p3)) =
                        (point_iter.next(), point_iter.next(), point_iter.next())
                    {
                        triangles.push(StaticTriangle2d::new(
                            remaining_polygon[*p1],
                            remaining_polygon[*p2],
                            remaining_polygon[*p3],
                        ));
                    }
                    triangles
                })
                .collect::<Vec<_>>()
            }),
            PolygonPath::None => [
                vec![StaticTriangle2d::new(*self.p1(), *self.p2(), *self.p3())],
                vec![],
            ],
        }
    }
}

mod cut;

#[derive(Debug, Clone)]
pub struct TrianglePointIterator<'a, T: Triangle2d> {
    triangle: &'a T,
    state: TriangleIteratorState,
}

#[derive(Default, Debug, Copy, Clone)]
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
    type Item = &'a StaticPoint2d;

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
    fn p1(&self) -> &StaticPoint2d {
        match self.side {
            TriangleSide::S1 => self.triangle.p1(),
            TriangleSide::S2 => self.triangle.p2(),
            TriangleSide::S3 => self.triangle.p3(),
        }
    }

    fn p2(&self) -> &StaticPoint2d {
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

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub enum TriangleSide {
    S1 = 0,
    S2 = 1,
    S3 = 2,
}

impl TriangleSide {
    pub fn start_corner(&self) -> TriangleCornerPoint {
        match self {
            TriangleSide::S1 => TriangleCornerPoint::P1,
            TriangleSide::S2 => TriangleCornerPoint::P2,
            TriangleSide::S3 => TriangleCornerPoint::P3,
        }
    }
    pub fn end_corner(&self) -> TriangleCornerPoint {
        match self {
            TriangleSide::S1 => TriangleCornerPoint::P2,
            TriangleSide::S2 => TriangleCornerPoint::P3,
            TriangleSide::S3 => TriangleCornerPoint::P1,
        }
    }
}

impl Into<usize> for TriangleSide {
    fn into(self) -> usize {
        match self {
            TriangleSide::S1 => 0,
            TriangleSide::S2 => 1,
            TriangleSide::S3 => 2,
        }
    }
}

impl Into<TriangleSide> for usize {
    fn into(self) -> TriangleSide {
        match self {
            0 => TriangleSide::S1,
            1 => TriangleSide::S2,
            2 => TriangleSide::S3,
            other => panic!("Out of range: Try convert index {other} to Triangle side"),
        }
    }
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
/*
fn pt_is_outside(pattern: &[SideOfLine; 3]) -> bool {
    pattern.iter().any(|s| *s == SideOfLine::Right)
}
*/
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub enum TriangleCornerPoint {
    P1 = 0,
    P2 = 1,
    P3 = 2,
}

#[derive(Default, Eq, PartialEq, Debug, Copy, Clone)]
enum TriangleCornerIteratorState {
    #[default]
    Begin,
    P1,
    P2,
    P3,
    Done,
}

impl TriangleCornerIteratorState {
    fn corner(&self) -> Option<TriangleCornerPoint> {
        match self {
            TriangleCornerIteratorState::Begin => None,
            TriangleCornerIteratorState::P1 => Some(TriangleCornerPoint::P1),
            TriangleCornerIteratorState::P2 => Some(TriangleCornerPoint::P2),
            TriangleCornerIteratorState::P3 => Some(TriangleCornerPoint::P3),
            TriangleCornerIteratorState::Done => None,
        }
    }
    fn move_forward(self) -> Self {
        match self {
            TriangleCornerIteratorState::Begin => TriangleCornerIteratorState::P1,
            TriangleCornerIteratorState::P1 => TriangleCornerIteratorState::P2,
            TriangleCornerIteratorState::P2 => TriangleCornerIteratorState::P3,
            TriangleCornerIteratorState::P3 => TriangleCornerIteratorState::Done,
            TriangleCornerIteratorState::Done => TriangleCornerIteratorState::Done,
        }
    }
}

#[derive(Default)]
pub struct TriangleCornerIterator(TriangleCornerIteratorState);

impl Iterator for TriangleCornerIterator {
    type Item = TriangleCornerPoint;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 = self.0.move_forward();
        self.0.corner()
    }
}
