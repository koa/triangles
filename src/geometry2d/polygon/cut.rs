use crate::geometry2d::triangle::TriangleSide;
use crate::primitives::Number;

#[derive(Debug)]
pub enum PolygonPath {
    Enclosed,
    CutSegments(Vec<CutSegment>),
    None,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PointPolygonRelationship {
    Inside,
    Outside,
    OnEdge,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct CutSegment {
    copy_points: PointRange,
    start_cut: LineCutIdx,
    end_cut: LineCutIdx,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
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
    pub fn iter(&self) -> PointRangeIterator {
        match self {
            PointRange::None => PointRangeIterator::None,
            PointRange::SingleRange {
                first_idx,
                last_idx,
            } => PointRangeIterator::SingleRange {
                next_value: Some(*first_idx),
                last_idx: *last_idx,
                forward: true,
            },
            PointRange::WarpAround {
                first_idx,
                last_idx,
                point_count,
            } => PointRangeIterator::WarpAround {
                last_idx: *last_idx,
                point_count: *point_count,
                next_value: Some(*first_idx),
                forward: true,
            },
        }
    }
    pub fn reverse_iter(&self) -> PointRangeIterator {
        match self {
            PointRange::None => PointRangeIterator::None,
            PointRange::SingleRange {
                first_idx,
                last_idx,
            } => PointRangeIterator::SingleRange {
                last_idx: *first_idx,
                next_value: Some(*last_idx),
                forward: false,
            },
            PointRange::WarpAround {
                first_idx,
                last_idx,
                point_count,
            } => PointRangeIterator::WarpAround {
                last_idx: *first_idx,
                point_count: *point_count,
                next_value: Some(*last_idx),
                forward: false,
            },
        }
    }
}

pub enum PointRangeIterator {
    None,
    SingleRange {
        last_idx: usize,
        next_value: Option<usize>,
        forward: bool,
    },
    WarpAround {
        last_idx: usize,
        point_count: usize,
        next_value: Option<usize>,
        forward: bool,
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
                forward,
            } => {
                if let Some(value) = *next_value {
                    *next_value = if *forward {
                        if value < *last_idx {
                            Some(value + 1)
                        } else {
                            None
                        }
                    } else if value > *last_idx {
                        Some(value - 1)
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
                forward,
            } => {
                if let Some(value) = *next_value {
                    *next_value = if *forward {
                        if value == *last_idx {
                            None
                        } else if value < *point_count - 1 {
                            Some(value + 1)
                        } else {
                            Some(0)
                        }
                    } else if value == *last_idx {
                        None
                    } else if value == 0 {
                        Some(*point_count - 1)
                    } else {
                        Some(value - 1)
                    };
                    Some(value)
                } else {
                    None
                }
            }
        }
    }
}

#[test]
fn test_point_range() {
    let mut iterator = PointRange::WarpAround {
        first_idx: 2,
        last_idx: 0,
        point_count: 3,
    }
    .iter();
    assert_eq!(Some(2), iterator.next());
    assert_eq!(Some(0), iterator.next());
    assert_eq!(None, iterator.next());
}

impl CutSegment {
    pub fn new(copy_points: PointRange, start_cut: LineCutIdx, end_cut: LineCutIdx) -> Self {
        Self {
            copy_points,
            start_cut,
            end_cut,
        }
    }

    #[inline]
    pub fn start_cut(&self) -> &LineCutIdx {
        &self.start_cut
    }
    #[inline]
    pub fn end_cut(&self) -> &LineCutIdx {
        &self.end_cut
    }
    #[inline]
    pub fn copy_points(&self) -> &PointRange {
        &self.copy_points
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct LineCutIdx {
    start_pt_idx: usize,
    triangle_line_idx: TriangleSide,
    polygon_pos: Number,
    triangle_pos: Number,
}

impl LineCutIdx {
    pub fn new(
        start_pt_idx: usize,
        triangle_line_idx: TriangleSide,
        polygon_pos: Number,
        triangle_pos: Number,
    ) -> Self {
        Self {
            start_pt_idx,
            triangle_line_idx,
            polygon_pos,
            triangle_pos,
        }
    }

    pub fn start_pt_idx(&self) -> usize {
        self.start_pt_idx
    }
    pub fn triangle_line_idx(&self) -> TriangleSide {
        self.triangle_line_idx
    }
    pub fn polygon_pos(&self) -> Number {
        self.polygon_pos
    }
    pub fn triangle_pos(&self) -> Number {
        self.triangle_pos
    }
}
