use crate::generic_triangle::{TriangleCornerPoint, TriangleSide};
use crate::geometry3d::line::referenced_line::ReferencedLine;
use crate::geometry3d::plane::{InvalidPlane, Plane3d};
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
    fn sides(&self) -> [ReferencedLine<P>; 3] {
        let [p1, p2, p3] = self.points();
        [
            ReferencedLine::new(p1, p2),
            ReferencedLine::new(p2, p3),
            ReferencedLine::new(p3, p1),
        ]
    }
    fn calculate_plane(&self) -> Result<Plane3d, InvalidPlane> {
        let [p1, p2, p3] = self.points();
        Plane3d::from_points(p1.coordinates(), p2.coordinates(), p3.coordinates())
    }
}
