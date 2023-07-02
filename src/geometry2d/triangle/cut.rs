use std::cmp::Ordering;
use std::fmt::Debug;

use num_traits::{One, Zero};

use crate::geometry2d::polygon::cut::{CutSegment, LineCutIdx};
use crate::geometry2d::triangle::{RelativeLinePosition, TriangleCornerPoint, TriangleSide};
use crate::geometry2d::{Polygon2d, Triangle2d};
use crate::primitives::Number;

impl RelativeLinePosition {
    fn relative_pt_along(&self) -> Number {
        match self {
            RelativeLinePosition::Start => Number::zero(),
            RelativeLinePosition::End => Number::one(),
            RelativeLinePosition::Crossing { along, .. } => *along,
        }
    }
}

struct WalkIndex<'a, P: Polygon2d + Debug, T: Triangle2d> {
    triangle: &'a T,
    cut_polygon: &'a P,
    segments: &'a [CutSegment],
    edge_points: Vec<TriangleEdgePoint<SegmentAlongSide<'a>>>,
    edge_peers: Vec<Option<usize>>,
}

enum TrianglePolygonPosition {
    TrianglePoint { idx: TriangleCornerPoint },
    PolygonPoint { idx: u32 },
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
enum WalkDirection {
    Forward,
    Backward,
}

trait PointAlongTriangleSide: Ord + Eq + Debug {
    fn side(&self) -> TriangleSide;
    fn along(&self) -> Number;
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum TriangleEdgePoint<P: PointAlongTriangleSide> {
    Corner(TriangleCornerPoint),
    AlongSide(P),
}
pub enum TraceResultPoint {
    Corner(TriangleCornerPoint),
    PolygonPoint(usize),
    CrossPoint {
        triangle_side: TriangleSide,
        polygon_side: usize,
        along_triangle: Number,
        along_polygon: Number,
    },
}

impl<P: PointAlongTriangleSide> Ord for TriangleEdgePoint<P> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.full_cmp(other)
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct SegmentAlongSide<'a> {
    segment: &'a CutSegment,
    direction: WalkDirection,
}

impl<'a> SegmentAlongSide<'a> {
    #[inline]
    fn front_cut(&self) -> &LineCutIdx {
        match self.direction {
            WalkDirection::Forward => self.segment.start_cut(),
            WalkDirection::Backward => self.segment.end_cut(),
        }
    }
}

impl<'a> PointAlongTriangleSide for SegmentAlongSide<'a> {
    fn side(&self) -> TriangleSide {
        self.front_cut().triangle_line_idx()
    }

    fn along(&self) -> Number {
        self.front_cut().triangle_pos()
    }
}

impl<P: PointAlongTriangleSide + Debug> TriangleEdgePoint<P> {
    fn full_cmp(&self, other: &Self) -> Ordering {
        match self {
            TriangleEdgePoint::Corner(my_corner) => match other {
                TriangleEdgePoint::Corner(other_corner) => my_corner.cmp(other_corner),
                TriangleEdgePoint::AlongSide(other_side) => {
                    if *my_corner as usize > other_side.side() as usize {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                }
            },
            TriangleEdgePoint::AlongSide(my_side) => match other {
                TriangleEdgePoint::Corner(other_corner) => {
                    if *other_corner as usize > my_side.side() as usize {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
                TriangleEdgePoint::AlongSide(other_side) => {
                    let side_cmp = my_side.side().cmp(&other_side.side());
                    if side_cmp != Ordering::Equal {
                        side_cmp
                    } else {
                        let along_cmp = my_side.along().cmp(&other_side.along());
                        if along_cmp != Ordering::Equal {
                            along_cmp
                        } else {
                            my_side.cmp(other_side)
                        }
                    }
                }
            },
        }
    }
}

impl<P: PointAlongTriangleSide> PartialOrd for TriangleEdgePoint<P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.full_cmp(other))
    }
}

impl<'a, P: Polygon2d + Debug, T: Triangle2d> WalkIndex<'a, P, T> {
    pub fn new(triangle: &'a T, cut_polygon: &'a P, segments: &'a [CutSegment]) -> Self {
        let mut edge_points: Vec<TriangleEdgePoint<SegmentAlongSide>> =
            Vec::with_capacity(3 + 2 * segments.len());
        edge_points.push(TriangleEdgePoint::Corner(TriangleCornerPoint::P1));
        edge_points.push(TriangleEdgePoint::Corner(TriangleCornerPoint::P2));
        edge_points.push(TriangleEdgePoint::Corner(TriangleCornerPoint::P3));
        for segment in segments {
            edge_points.push(TriangleEdgePoint::AlongSide(SegmentAlongSide {
                segment,
                direction: WalkDirection::Forward,
            }));
            edge_points.push(TriangleEdgePoint::AlongSide(SegmentAlongSide {
                segment,
                direction: WalkDirection::Backward,
            }));
        }
        edge_points.sort();
        for (idx, p) in edge_points.iter().enumerate() {
            println!("[{idx}]: {p:?}");
        }
        let mut edge_peers = vec![None; edge_points.len()];
        for (idx, p) in edge_points.iter().enumerate() {
            if edge_peers
                .get(idx)
                .map_or(false, |i: &Option<usize>| i.is_some())
            {
                continue;
            }
            if let TriangleEdgePoint::AlongSide(s1) = p {
                for (idx2, candidate) in edge_points[idx + 1..].iter().enumerate() {
                    if let TriangleEdgePoint::AlongSide(s2) = candidate {
                        if s1.segment == s2.segment {
                            let idx2 = idx2 + 1 + idx;
                            edge_peers[idx] = Some(idx2);
                            edge_peers[idx2] = Some(idx);
                            break;
                        }
                    }
                }
            }
        }
        Self {
            triangle,
            cut_polygon,
            segments,
            edge_points,
            edge_peers,
        }
    }
    pub fn next_pos(&self, current_postion: usize) -> usize {
        (current_postion + 1) % self.edge_points.len()
    }
    pub fn entry(&'a self, idx: usize) -> &'a TriangleEdgePoint<SegmentAlongSide<'a>> {
        self.edge_points
            .get(idx)
            .expect("current_position out of range")
    }
    pub fn peer_of(&self, idx: usize) -> Option<usize> {
        self.edge_peers.get(idx).copied().unwrap_or_default()
    }
}

impl Default for TrianglePolygonPosition {
    fn default() -> Self {
        Self::TrianglePoint {
            idx: TriangleCornerPoint::P1,
        }
    }
}
pub fn walk_shape_recursive<P: Polygon2d + Debug, T: Triangle2d>(
    triangle: &T,
    cut_polygon: &P,
    segments: &[CutSegment],
) -> Vec<Vec<TraceResultPoint>> {
    let mut startpoint_stack = Vec::with_capacity(3);
    startpoint_stack.push(0);
    let walker = WalkIndex::new(triangle, cut_polygon, segments);
    let mut result = Vec::with_capacity(1);
    while let Some(idx) = startpoint_stack.pop() {
        let mut trace_path = Vec::new();
        let mut last_idx = None;
        let mut next_idx = idx;
        loop {
            let x = last_idx.is_none() || next_idx != idx;
            println!("{last_idx:?},{next_idx}:{x}");
            if !x {
                break;
            }
            let current_idx = match walker.entry(next_idx) {
                TriangleEdgePoint::Corner(p) => {
                    trace_path.push(TraceResultPoint::Corner(*p));
                    next_idx
                }
                TriangleEdgePoint::AlongSide(side) => {
                    let (c1, i, c2) = match side.direction {
                        WalkDirection::Forward => (
                            side.segment.start_cut(),
                            side.segment.copy_points().iter(),
                            side.segment.end_cut(),
                        ),
                        WalkDirection::Backward => (
                            side.segment.end_cut(),
                            side.segment.copy_points().reverse_iter(),
                            side.segment.start_cut(),
                        ),
                    };
                    trace_path.push(TraceResultPoint::CrossPoint {
                        triangle_side: c1.triangle_line_idx(),
                        polygon_side: c1.start_pt_idx(),
                        along_triangle: c1.triangle_pos(),
                        along_polygon: c1.polygon_pos(),
                    });
                    for polygon_point in i {
                        trace_path.push(TraceResultPoint::PolygonPoint(polygon_point));
                    }
                    trace_path.push(TraceResultPoint::CrossPoint {
                        triangle_side: c2.triangle_line_idx(),
                        polygon_side: c2.start_pt_idx(),
                        along_triangle: c2.triangle_pos(),
                        along_polygon: c2.polygon_pos(),
                    });
                    let next_idx = walker.peer_of(next_idx).expect("data error");
                    if last_idx.is_some() {
                        startpoint_stack.push(next_idx);
                    }
                    next_idx
                }
            };
            last_idx = Some(current_idx);
            next_idx = walker.next_pos(current_idx);
        }
        trace_path.truncate(trace_path.len());
        result.push(trace_path);
    }
    result
}
