use std::{
    fmt::{Debug, Formatter},
    ops::{Add, Sub},
};

use triangulate::Vertex;

use crate::geometry2d::point::Point2d;
use crate::geometry2d::vector::Vector2d;
use crate::primitives::{Float, Number};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StaticPoint2d {
    pub x: Number,
    pub y: Number,
}

impl Into<Vector2d> for StaticPoint2d {
    fn into(self) -> Vector2d {
        Vector2d {
            x: self.x,
            y: self.y,
        }
    }
}

impl Point2d for StaticPoint2d {
    fn coordinates(&self) -> StaticPoint2d {
        *self
    }
}

impl Add<Vector2d> for StaticPoint2d {
    type Output = Self;

    fn add(self, rhs: Vector2d) -> Self::Output {
        StaticPoint2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for StaticPoint2d {
    type Output = Vector2d;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2d::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Debug for StaticPoint2d {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{})", self.x, self.y))
    }
}

impl From<(Number, Number)> for StaticPoint2d {
    fn from((x, y): (Number, Number)) -> Self {
        Self { x, y }
    }
}

impl From<(Float, Float)> for StaticPoint2d {
    fn from((x, y): (Float, Float)) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl Vertex for StaticPoint2d {
    type Coordinate = Number;

    fn x(&self) -> Self::Coordinate {
        self.x
    }

    fn y(&self) -> Self::Coordinate {
        self.y
    }
}
