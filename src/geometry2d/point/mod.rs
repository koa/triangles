use std::fmt::Debug;

use triangulate::Vertex;

pub use static_point::StaticPoint2d;

use crate::primitives::Number;

pub(crate) mod bounding_box;
mod static_point;

pub trait Point2d: Sized + Debug + PartialEq + Clone + Vertex {
    fn x(&self) -> Number {
        self.coordinates().x
    }
    fn y(&self) -> Number {
        self.coordinates().y
    }
    fn coordinates(&self) -> StaticPoint2d;
    fn dist_square<P: Point2d>(&self, other: &P) -> Number {
        let x_dist = Point2d::x(self) - Point2d::x(other);
        let y_dist = Point2d::y(self) - Point2d::y(other);
        x_dist * x_dist + y_dist * y_dist
    }
}
