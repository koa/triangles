use std::ops::Mul;

use crate::primitives::Number;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vector2d {
    x: Number,
    y: Number,
}

impl Vector2d {
    #[inline]
    pub fn x(&self) -> Number {
        self.x
    }
    #[inline]
    pub fn y(&self) -> Number {
        self.y
    }
    pub fn new(x: Number, y: Number) -> Self {
        Self { x, y }
    }
}

impl Mul<Number> for Vector2d {
    type Output = Self;

    fn mul(self, rhs: Number) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
