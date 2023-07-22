use std::fmt::Debug;

use triangulate::Vertex;

pub use static_point::StaticPoint2d;

use crate::primitives::Number;

pub(crate) mod bounding_box;
mod static_point;

pub trait Point2d: Sized + Debug + PartialEq + Clone + Vertex {
    /*fn x(&self) -> Number {
        self.coordinates().x
    }
    fn y(&self) -> Number {
        self.coordinates().y
    }*/
    fn coordinates(&self) -> StaticPoint2d;
    fn dist_square<P: Point2d>(&self, other: &P) -> Number {
        let StaticPoint2d { x: sx, y: sy } = self.coordinates();
        let StaticPoint2d { x: ox, y: oy } = other.coordinates();
        let x_dist = sx - ox;
        let y_dist = sy - oy;
        x_dist * x_dist + y_dist * y_dist
    }
}
