use std::ops::AddAssign;
use std::{fmt::Debug, ops::Add};

use crate::prelude::StaticPoint2d;
use crate::primitives::Number;

#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
pub enum BoundingBox2d {
    #[default]
    Empty,
    Box(BoundingBox2dValues),
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Debug)]
pub struct BoundingBox2dValues {
    pub min_x: Number,
    pub min_y: Number,
    pub max_x: Number,
    pub max_y: Number,
}

impl BoundingBox2dValues {
    pub fn min_x(&self) -> Number {
        self.min_x
    }
    pub fn min_y(&self) -> Number {
        self.min_y
    }
    pub fn max_x(&self) -> Number {
        self.max_x
    }
    pub fn max_y(&self) -> Number {
        self.max_y
    }
    pub fn width(&self) -> Number {
        self.max_x - self.min_x
    }
    pub fn height(&self) -> Number {
        self.max_y - self.min_y
    }
    pub fn new(min_x: Number, min_y: Number, max_x: Number, max_y: Number) -> Self {
        Self {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }
    pub fn expand(&self, s: Number) -> Self {
        let expand_horizontal = self.width() * s / 2.0;
        let expand_vertical = self.height() * s / 2.0;
        Self {
            min_x: self.min_x - expand_horizontal,
            min_y: self.min_y - expand_vertical,
            max_x: self.max_x + expand_horizontal,
            max_y: self.max_y + expand_vertical,
        }
    }
}

impl Add for BoundingBox2d {
    type Output = BoundingBox2d;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (BoundingBox2d::Empty, right) => right,
            (left, BoundingBox2d::Empty) => left,
            (
                BoundingBox2d::Box(BoundingBox2dValues {
                    min_x: min_x1,
                    min_y: min_y1,
                    max_x: max_x1,
                    max_y: max_y1,
                }),
                BoundingBox2d::Box(BoundingBox2dValues {
                    min_x: min_x2,
                    min_y: min_y2,
                    max_x: max_x2,
                    max_y: max_y2,
                }),
            ) => BoundingBox2d::Box(BoundingBox2dValues {
                min_x: min_x1.min(min_x2),
                min_y: min_y1.min(min_y2),
                max_x: max_x1.max(max_x2),
                max_y: max_y1.max(max_y2),
            }),
        }
    }
}

impl AddAssign for BoundingBox2d {
    fn add_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (
                BoundingBox2d::Box(BoundingBox2dValues {
                    min_x: min_x1,
                    min_y: min_y1,
                    max_x: max_x1,
                    max_y: max_y1,
                }),
                BoundingBox2d::Box(BoundingBox2dValues {
                    min_x: min_x2,
                    min_y: min_y2,
                    max_x: max_x2,
                    max_y: max_y2,
                }),
            ) => {
                *min_x1 = (*min_x1).min(min_x2);
                *min_y1 = (*min_y1).min(min_y2);
                *max_x1 = (*max_x1).max(max_x2);
                *max_y1 = (*max_y1).max(max_y2);
            }
            (_, BoundingBox2d::Empty) => {}
            (my, other) => *my = other,
        }
    }
}

impl Add<StaticPoint2d> for BoundingBox2d {
    type Output = BoundingBox2d;

    fn add(self, rhs: StaticPoint2d) -> Self::Output {
        let StaticPoint2d { x, y } = rhs;
        match self {
            BoundingBox2d::Empty => BoundingBox2d::Box(BoundingBox2dValues {
                min_x: x,
                min_y: y,
                max_x: x,
                max_y: y,
            }),
            BoundingBox2d::Box(BoundingBox2dValues {
                min_x,
                min_y,
                max_x,
                max_y,
            }) => BoundingBox2d::Box(BoundingBox2dValues {
                min_x: min_x.min(x),
                min_y: min_y.min(y),
                max_x: max_x.max(x),
                max_y: max_y.max(y),
            }),
        }
    }
}

impl AddAssign<StaticPoint2d> for BoundingBox2d {
    fn add_assign(&mut self, rhs: StaticPoint2d) {
        let StaticPoint2d { x, y } = rhs;
        match self {
            BoundingBox2d::Box(BoundingBox2dValues {
                min_x,
                min_y,
                max_x,
                max_y,
            }) => {
                *min_x = (*min_x).min(x);
                *min_y = (*min_y).min(y);
                *max_x = (*max_x).max(x);
                *max_y = (*max_y).max(y);
            }
            my => {
                *my = BoundingBox2d::Box(BoundingBox2dValues {
                    min_x: x,
                    min_y: y,
                    max_x: x,
                    max_y: y,
                })
            }
        };
    }
}
