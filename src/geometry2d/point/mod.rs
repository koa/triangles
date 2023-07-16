pub use static_point::StaticPoint2d;

use crate::primitives::Number;

pub(crate) mod bounding_box;
mod static_point;

pub trait Point2d: Sized {
    fn x(&self) -> Number;
    fn y(&self) -> Number;
    fn coordinates(&self) -> StaticPoint2d {
        StaticPoint2d {
            x: self.x(),
            y: self.y(),
        }
    }
    fn dist_square<P: Point2d>(&self, other: &P) -> Number {
        let x_dist = self.x() - other.x();
        let y_dist = self.y() - other.y();
        x_dist * x_dist + y_dist * y_dist
    }
}
