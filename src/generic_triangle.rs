use std::fmt::{Display, Formatter, Write};
use thiserror::Error;

/// triangle properties independent of dimension count (corners and edges)

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub enum TriangleCornerPoint {
    P1 = 0,
    P2 = 1,
    P3 = 2,
}
#[derive(Debug, Error)]
pub struct TriangleIndexOutOfBoundsError {
    idx: usize,
}

impl Display for TriangleIndexOutOfBoundsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Triangle index out of bounds: {}", self.idx)
    }
}

impl TryFrom<usize> for TriangleCornerPoint {
    type Error = TriangleIndexOutOfBoundsError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TriangleCornerPoint::P1),
            1 => Ok(TriangleCornerPoint::P2),
            2 => Ok(TriangleCornerPoint::P3),
            idx => Err(TriangleIndexOutOfBoundsError { idx }),
        }
    }
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

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub enum TriangleSide {
    S1 = 0,
    S2 = 1,
    S3 = 2,
}

impl Display for TriangleSide {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TriangleSide::S1 => f.write_str("s1"),
            TriangleSide::S2 => f.write_str("s2"),
            TriangleSide::S3 => f.write_str("s3"),
        }
    }
}

impl TriangleSide {
    #[inline]
    pub fn start_corner(&self) -> TriangleCornerPoint {
        match self {
            TriangleSide::S1 => TriangleCornerPoint::P1,
            TriangleSide::S2 => TriangleCornerPoint::P2,
            TriangleSide::S3 => TriangleCornerPoint::P3,
        }
    }
    #[inline]
    pub fn end_corner(&self) -> TriangleCornerPoint {
        match self {
            TriangleSide::S1 => TriangleCornerPoint::P2,
            TriangleSide::S2 => TriangleCornerPoint::P3,
            TriangleSide::S3 => TriangleCornerPoint::P1,
        }
    }
}

impl From<TriangleSide> for usize {
    fn from(val: TriangleSide) -> Self {
        match val {
            TriangleSide::S1 => 0,
            TriangleSide::S2 => 1,
            TriangleSide::S3 => 2,
        }
    }
}

impl From<usize> for TriangleSide {
    fn from(value: usize) -> Self {
        match value {
            0 => TriangleSide::S1,
            1 => TriangleSide::S2,
            2 => TriangleSide::S3,
            other => panic!("Out of range: Try convert index {other} to Triangle side"),
        }
    }
}
