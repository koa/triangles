use std::ops::AddAssign;
use std::{fmt::Debug, ops::Add};

use crate::prelude::{Point3d, Vector3d};
use crate::primitives::Number;

#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
pub enum BoundingBox3d {
    #[default]
    Empty,
    Box(BoundingBox3dValues),
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Debug)]
pub struct BoundingBox3dValues {
    pub min_x: Number,
    pub min_y: Number,
    pub min_z: Number,
    pub max_x: Number,
    pub max_y: Number,
    pub max_z: Number,
}

impl BoundingBox3dValues {
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
}

impl Add for BoundingBox3d {
    type Output = BoundingBox3d;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (BoundingBox3d::Empty, right) => right,
            (left, BoundingBox3d::Empty) => left,
            (
                BoundingBox3d::Box(BoundingBox3dValues {
                    min_x: min_x1,
                    min_y: min_y1,
                    min_z: min_z1,
                    max_x: max_x1,
                    max_y: max_y1,
                    max_z: max_z1,
                }),
                BoundingBox3d::Box(BoundingBox3dValues {
                    min_x: min_x2,
                    min_y: min_y2,
                    min_z: min_z2,
                    max_x: max_x2,
                    max_y: max_y2,
                    max_z: max_z2,
                }),
            ) => BoundingBox3d::Box(BoundingBox3dValues {
                min_x: min_x1.min(min_x2),
                min_y: min_y1.min(min_y2),
                min_z: min_z1.min(min_z2),
                max_x: max_x1.max(max_x2),
                max_y: max_y1.max(max_y2),
                max_z: max_z1.max(max_z2),
            }),
        }
    }
}

impl AddAssign for BoundingBox3d {
    fn add_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (
                BoundingBox3d::Box(BoundingBox3dValues {
                    min_x: min_x1,
                    min_y: min_y1,
                    min_z: min_z1,
                    max_x: max_x1,
                    max_y: max_y1,
                    max_z: max_z1,
                }),
                BoundingBox3d::Box(BoundingBox3dValues {
                    min_x: min_x2,
                    min_y: min_y2,
                    min_z: min_z2,
                    max_x: max_x2,
                    max_y: max_y2,
                    max_z: max_z2,
                }),
            ) => {
                *min_x1 = (*min_x1).min(min_x2);
                *min_y1 = (*min_y1).min(min_y2);
                *min_z1 = (*min_z1).min(min_z2);
                *max_x1 = (*max_x1).max(max_x2);
                *max_y1 = (*max_y1).max(max_y2);
                *max_z1 = (*max_z1).max(max_z2);
            }
            (_, BoundingBox3d::Empty) => {}
            (my, other) => *my = other,
        }
    }
}

impl<P: Point3d> Add<P> for BoundingBox3d {
    type Output = BoundingBox3d;

    fn add(self, rhs: P) -> Self::Output {
        let Vector3d { x, y, z } = rhs.coordinates();
        match self {
            BoundingBox3d::Empty => BoundingBox3d::Box(BoundingBox3dValues {
                min_x: x,
                min_y: y,
                min_z: z,
                max_x: x,
                max_y: y,
                max_z: z,
            }),
            BoundingBox3d::Box(BoundingBox3dValues {
                min_x,
                min_y,
                min_z,
                max_x,
                max_y,
                max_z,
            }) => BoundingBox3d::Box(BoundingBox3dValues {
                min_x: min_x.min(x),
                min_y: min_y.min(y),
                min_z: min_z.min(z),
                max_x: max_x.max(x),
                max_y: max_y.max(y),
                max_z: max_z.max(z),
            }),
        }
    }
}

impl<P: Point3d> AddAssign<P> for BoundingBox3d {
    fn add_assign(&mut self, rhs: P) {
        let Vector3d { x, y, z } = rhs.coordinates();
        match self {
            BoundingBox3d::Box(BoundingBox3dValues {
                min_x,
                min_y,
                min_z,
                max_x,
                max_y,
                max_z,
            }) => {
                *min_x = (*min_x).min(x);
                *min_y = (*min_y).min(y);
                *min_z = (*min_z).min(z);
                *max_x = (*max_x).max(x);
                *max_y = (*max_y).max(y);
                *max_z = (*max_z).max(z);
            }
            my => {
                *my = BoundingBox3d::Box(BoundingBox3dValues {
                    min_x: x,
                    min_y: y,
                    min_z: z,
                    max_x: x,
                    max_y: y,
                    max_z: z,
                })
            }
        };
    }
}
