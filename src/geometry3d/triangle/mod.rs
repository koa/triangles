use crate::generic_triangle::TriangleCornerPoint;
use crate::geometry3d::point::Point3d;

pub trait Triangle3d<P: Point3d>: Sized + PartialEq {
    fn get_point(&self, p: TriangleCornerPoint) -> &P;
    fn points(&self) -> [&P; 3];
}
