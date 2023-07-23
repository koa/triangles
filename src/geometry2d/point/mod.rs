use std::fmt::Debug;

use triangulate::Vertex;

pub use static_point::StaticPoint2d;

use crate::primitives::Number;

pub(crate) mod bounding_box;
mod static_point;

pub trait Point2d: Sized + Debug + PartialEq + Clone + Vertex {
    fn coordinates(&self) -> StaticPoint2d;
    fn dist_square<P: Point2d>(&self, other: &P) -> Number {
        let StaticPoint2d { x: sx, y: sy } = self.coordinates();
        let StaticPoint2d { x: ox, y: oy } = other.coordinates();
        let x_dist = sx - ox;
        let y_dist = sy - oy;
        x_dist * x_dist + y_dist * y_dist
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum PointOrPoint<P1: Point2d, P2: Point2d> {
    P1(P1),
    P2(P2),
}

impl<P1: Point2d, P2: Point2d> Vertex for PointOrPoint<P1, P2> {
    type Coordinate = Number;

    fn x(&self) -> Self::Coordinate {
        self.coordinates().x
    }

    fn y(&self) -> Self::Coordinate {
        self.coordinates().y
    }
}

impl<P1: Point2d, P2: Point2d> Point2d for PointOrPoint<P1, P2> {
    fn coordinates(&self) -> StaticPoint2d {
        match self {
            PointOrPoint::P1(p) => p.coordinates(),
            PointOrPoint::P2(p) => p.coordinates(),
        }
    }
}
