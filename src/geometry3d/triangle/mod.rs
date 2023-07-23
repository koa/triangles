use crate::generic_triangle::{TriangleCornerPoint, TriangleSide};
use crate::geometry3d::line::referenced_line::ReferencedLine;
use crate::geometry3d::point::Point3d;

pub trait Triangle3d<P: Point3d>: Sized + PartialEq {
    fn get_point(&self, p: TriangleCornerPoint) -> &P;
    fn points(&self) -> [&P; 3];
    fn get_side(&self, side: TriangleSide) -> ReferencedLine<P> {
        ReferencedLine::new(
            self.get_point(side.start_corner()),
            self.get_point(side.end_corner()),
        )
    }
}
