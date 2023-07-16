use std::ops::AddAssign;
use std::{fmt::Debug, ops::Add};

#[cfg(test)]
use svg::node::{
    element::{
        path::{Data, Parameters},
        Path, SVG,
    },
    Value,
};

use crate::prelude::StaticPoint2d;
use crate::primitives::Number;

#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
pub enum BoundingBox {
    #[default]
    Empty,
    Box(BoundingBoxValues),
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Debug)]
pub struct BoundingBoxValues {
    min_x: Number,
    min_y: Number,
    max_x: Number,
    max_y: Number,
}

impl BoundingBoxValues {
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

#[cfg(test)]
#[derive(Default, Copy, Clone)]
pub enum BoundingBoxSvg {
    #[default]
    Empty,
    Box {
        min_x: f32,
        min_y: f32,
        max_x: f32,
        max_y: f32,
    },
}

#[cfg(test)]
impl BoundingBoxSvg {
    pub fn apply(&self, point: &StaticPoint2d) -> Parameters {
        let x = point.x.0 as f32;
        let y = -point.y.0 as f32;
        match self {
            BoundingBoxSvg::Empty => Parameters::from((x, y)),
            BoundingBoxSvg::Box { min_x, min_y, .. } => Parameters::from((x - min_x, y - min_y)),
        }
    }
    pub fn plot_coordinates(&self, svg: SVG) -> SVG {
        if let BoundingBoxSvg::Box {
            min_x,
            min_y,
            max_x,
            max_y,
        } = self
        {
            let svg = if *min_x <= 0.0 && *max_x >= 0.0 {
                let min_pt = self.apply(&(0.0, *min_y as f64).into());
                let max_pt = self.apply(&(0.0, *max_y as f64).into());
                svg.add(
                    Path::new()
                        .set("fill", "none")
                        .set("stroke", "blue")
                        .set("stroke-width", 0.1)
                        .set("d", Data::new().move_to(min_pt).line_to(max_pt)),
                )
            } else {
                svg
            };
            if *min_y <= 0.0 && *max_y >= 0.0 {
                let min_pt = self.apply(&(*min_x as f64, 0.0).into());
                let max_pt = self.apply(&(*max_x as f64, 0.0).into());
                svg.add(
                    Path::new()
                        .set("fill", "none")
                        .set("stroke", "blue")
                        .set("stroke-width", 0.1)
                        .set("d", Data::new().move_to(min_pt).line_to(max_pt)),
                )
            } else {
                svg
            }
        } else {
            svg
        }
    }
}

#[cfg(test)]
#[inline]
fn point2svg(rhs: &StaticPoint2d) -> (f32, f32) {
    (rhs.x.0 as f32, -rhs.y.0 as f32)
}

impl Add for BoundingBox {
    type Output = BoundingBox;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (BoundingBox::Empty, right) => right,
            (left, BoundingBox::Empty) => left,
            (
                BoundingBox::Box(BoundingBoxValues {
                    min_x: min_x1,
                    min_y: min_y1,
                    max_x: max_x1,
                    max_y: max_y1,
                }),
                BoundingBox::Box(BoundingBoxValues {
                    min_x: min_x2,
                    min_y: min_y2,
                    max_x: max_x2,
                    max_y: max_y2,
                }),
            ) => BoundingBox::Box(BoundingBoxValues {
                min_x: min_x1.min(min_x2),
                min_y: min_y1.min(min_y2),
                max_x: max_x1.max(max_x2),
                max_y: max_y1.max(max_y2),
            }),
        }
    }
}

impl AddAssign for BoundingBox {
    fn add_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (
                BoundingBox::Box(BoundingBoxValues {
                    min_x: min_x1,
                    min_y: min_y1,
                    max_x: max_x1,
                    max_y: max_y1,
                }),
                BoundingBox::Box(BoundingBoxValues {
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
            (_, BoundingBox::Empty) => {}
            (my, other) => *my = other,
        }
    }
}

#[cfg(test)]
impl Add for BoundingBoxSvg {
    type Output = BoundingBoxSvg;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (BoundingBoxSvg::Empty, right) => right,
            (left, BoundingBoxSvg::Empty) => left,
            (
                BoundingBoxSvg::Box {
                    min_x: min_x1,
                    min_y: min_y1,
                    max_x: max_x1,
                    max_y: max_y1,
                },
                BoundingBoxSvg::Box {
                    min_x: min_x2,
                    min_y: min_y2,
                    max_x: max_x2,
                    max_y: max_y2,
                },
            ) => BoundingBoxSvg::Box {
                min_x: min_x1.min(min_x2),
                min_y: min_y1.min(min_y2),
                max_x: max_x1.max(max_x2),
                max_y: max_y1.max(max_y2),
            },
        }
    }
}

#[cfg(test)]
impl AddAssign for BoundingBoxSvg {
    fn add_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (
                BoundingBoxSvg::Box {
                    min_x: min_x1,
                    min_y: min_y1,
                    max_x: max_x1,
                    max_y: max_y1,
                },
                BoundingBoxSvg::Box {
                    min_x: min_x2,
                    min_y: min_y2,
                    max_x: max_x2,
                    max_y: max_y2,
                },
            ) => {
                *min_x1 = min_x1.min(min_x2);
                *min_y1 = min_y1.min(min_y2);
                *max_x1 = max_x1.max(max_x2);
                *max_y1 = max_y1.max(max_y2);
            }
            (_, BoundingBoxSvg::Empty) => {}
            (my, other) => *my = other,
        }
    }
}

impl Add<StaticPoint2d> for BoundingBox {
    type Output = BoundingBox;

    fn add(self, rhs: StaticPoint2d) -> Self::Output {
        let StaticPoint2d { x, y } = rhs;
        match self {
            BoundingBox::Empty => BoundingBox::Box(BoundingBoxValues {
                min_x: x,
                min_y: y,
                max_x: x,
                max_y: y,
            }),
            BoundingBox::Box(BoundingBoxValues {
                min_x,
                min_y,
                max_x,
                max_y,
            }) => BoundingBox::Box(BoundingBoxValues {
                min_x: min_x.min(x),
                min_y: min_y.min(y),
                max_x: max_x.max(x),
                max_y: max_y.max(y),
            }),
        }
    }
}

#[cfg(test)]
impl Add<StaticPoint2d> for BoundingBoxSvg {
    type Output = BoundingBoxSvg;

    fn add(self, rhs: StaticPoint2d) -> Self::Output {
        let (x, y) = point2svg(&rhs);
        match self {
            BoundingBoxSvg::Empty => BoundingBoxSvg::Box {
                min_x: x,
                min_y: y,
                max_x: x,
                max_y: y,
            },
            BoundingBoxSvg::Box {
                min_x,
                min_y,
                max_x,
                max_y,
            } => BoundingBoxSvg::Box {
                min_x: min_x.min(x),
                min_y: min_y.min(y),
                max_x: max_x.max(x),
                max_y: max_y.max(y),
            },
        }
    }
}

impl AddAssign<StaticPoint2d> for BoundingBox {
    fn add_assign(&mut self, rhs: StaticPoint2d) {
        let StaticPoint2d { x, y } = rhs;
        match self {
            BoundingBox::Box(BoundingBoxValues {
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
                *my = BoundingBox::Box(BoundingBoxValues {
                    min_x: x,
                    min_y: y,
                    max_x: x,
                    max_y: y,
                })
            }
        };
    }
}

#[cfg(test)]
impl AddAssign<StaticPoint2d> for BoundingBoxSvg {
    fn add_assign(&mut self, rhs: StaticPoint2d) {
        let (x, y) = point2svg(&rhs);
        match self {
            BoundingBoxSvg::Box {
                min_x,
                min_y,
                max_x,
                max_y,
            } => {
                *min_x = min_x.min(x);
                *min_y = min_y.min(y);
                *max_x = max_x.max(x);
                *max_y = max_y.max(y);
            }
            my => {
                *my = BoundingBoxSvg::Box {
                    min_x: x,
                    min_y: y,
                    max_x: x,
                    max_y: y,
                }
            }
        };
    }
}

#[cfg(test)]
impl From<BoundingBoxSvg> for Value {
    fn from(value: BoundingBoxSvg) -> Self {
        match value {
            BoundingBoxSvg::Empty => "".into(),
            BoundingBoxSvg::Box {
                min_x,
                min_y,
                max_x,
                max_y,
            } => {
                let span_x = max_x - min_x;
                let span_y = max_y - min_y;
                (0, 0, span_x, span_y).into()
            }
        }
    }
}
