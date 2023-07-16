use std::{
    fmt::{Debug, Formatter},
    ops::{Add, Sub},
};

use triangulate::Vertex;

use crate::geometry2d::vector::Vector2d;
use crate::primitives::{Float, Number};

pub(crate) mod bounding_box;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Point2d {
    pub x: Number,
    pub y: Number,
}

impl Point2d {
    pub fn x(&self) -> Number {
        self.x
    }
    pub fn y(&self) -> Number {
        self.y
    }
    pub fn dist_square(&self, other: &Point2d) -> Number {
        let x_dist = self.x - other.x;
        let y_dist = self.y - other.y;
        x_dist * x_dist + y_dist * y_dist
    }
}

impl Add<Vector2d> for Point2d {
    type Output = Self;

    fn add(self, rhs: Vector2d) -> Self::Output {
        Point2d {
            x: self.x + rhs.x(),
            y: self.y + rhs.y(),
        }
    }
}

impl Sub for Point2d {
    type Output = Vector2d;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2d::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Debug for Point2d {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{})", self.x, self.y))
    }
}

impl From<(Number, Number)> for Point2d {
    fn from((x, y): (Number, Number)) -> Self {
        Self { x, y }
    }
}
impl From<(Float, Float)> for Point2d {
    fn from((x, y): (Float, Float)) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl Vertex for Point2d {
    type Coordinate = Number;

    fn x(&self) -> Self::Coordinate {
        self.x
    }

    fn y(&self) -> Self::Coordinate {
        self.y
    }
}
