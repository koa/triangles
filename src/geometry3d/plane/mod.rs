use std::fmt::{Display, Formatter};

use num_traits::Zero;
use thiserror::Error;
use vek::Vec3;

use crate::geometry3d::line::static_line::StaticLine3d;
use crate::geometry3d::line::Line3d;
use crate::geometry3d::point::Point3d;
use crate::geometry3d::Vector3d;
use crate::prelude::Number;

pub mod projection;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Plane3d {
    normal: Vector3d,
    distance: Number,
}

impl Plane3d {
    pub fn from_points<P1: Into<Vector3d>, P2: Into<Vector3d>, P3: Into<Vector3d>>(
        p1: P1,
        p2: P2,
        p3: P3,
    ) -> Result<Plane3d, InvalidPlane> {
        let v1: Vector3d = p1.into();
        let v2: Vector3d = p2.into();
        let v3: Vector3d = p3.into();
        let normal = (v2 - v1).cross(v3 - v1);
        if normal.magnitude() == Number::zero() {
            return Err(InvalidPlane {});
        }

        let normal = normal.normalized();
        let distance = (normal.dot(v1));
        Ok(Self { normal, distance })
    }
    #[inline]
    pub fn normal(&self) -> Vector3d {
        self.normal
    }

    pub fn intersect_line_at<L: Line3d<P>, P: Point3d>(&self, line: &L) -> Number {
        let r0 = self.point_in_plane();
        let n = self.normal();
        let r1 = line.p1().coordinates();
        let a = line.direction();
        intersect_line(&r0, &n, &r1, &a)
    }

    pub fn relationship(&self, other: &Plane3d) -> PlaneCutRelationship {
        let direction = self.normal.cross(other.normal);
        if direction.magnitude_squared() == Number::zero() {
            if self.point_in_plane() == other.point_in_plane() {
                PlaneCutRelationship::Same
            } else {
                PlaneCutRelationship::Parallel
            }
        } else {
            let linedir_in_plane = direction.cross(self.normal);
            let point_in_plane = self.point_in_plane();
            let pos_in_line = intersect_line(
                &other.point_in_plane(),
                &other.normal,
                &point_in_plane,
                &linedir_in_plane,
            );
            let point = point_in_plane + linedir_in_plane * pos_in_line;
            PlaneCutRelationship::Line(StaticLine3d::new(point, direction))
        }
    }
    #[inline]
    fn point_in_plane(&self) -> Vec3<Number> {
        self.normal * self.distance
    }
    pub fn is_in_front<P: Point3d>(&self, p: &P) -> bool {
        self.point_distance(p) > Number::zero()
    }
    pub fn point_distance<P: Point3d>(&self, p: &P) -> Number {
        self.normal
            .dot(p.coordinates() - (self.normal * self.distance))
    }
}
fn intersect_line(
    plane_point: &Vec3<Number>,
    plane_normal: &Vec3<Number>,
    line_point: &Vec3<Number>,
    line_direction: &Vec3<Number>,
) -> Number {
    let div = plane_normal.dot(*line_direction);
    plane_normal.dot(*plane_point - *line_point) / div
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlaneCutRelationship {
    Parallel,
    Same,
    Line(StaticLine3d),
}

#[derive(Error, Debug, Eq, PartialEq, Copy, Clone)]
pub struct InvalidPlane {}

impl Display for InvalidPlane {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Plane")
    }
}

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;
    use vek::approx::relative_eq;
    use vek::Quaternion;

    use crate::geometry3d::line::static_line::StaticLine3d;
    use crate::geometry3d::plane::{InvalidPlane, Plane3d, PlaneCutRelationship};
    use crate::geometry3d::point::point_3d;
    use crate::prelude::AnyPolygon::StaticTrianglePolygon;
    use crate::prelude::Line3d;
    use crate::primitives::Number;

    #[test]
    fn test_rotate_normal() {
        let target = point_3d(0.0, 0.0, 1.0);
        let plane_normal = point_3d(1.0, 0.0, 0.0);
        let q = Quaternion::<Number>::rotation_from_to_3d(plane_normal, target);
        println!("q: {q:?}, m: {}", q.magnitude());
        let new_normal = q * plane_normal;
        println!("rotated normal {new_normal:?}");
        let x = q * point_3d(1.0, 1.0, 0.0);
        println!("rotated point {x:?}");
    }

    #[test]
    fn test_create_plane() {
        let p1 = Plane3d::from_points(
            point_3d(-1.0, -1.0, 0.0),
            point_3d(1.0, -1.0, 1.0),
            point_3d(1.0, -1.0, -1.0),
        )
        .unwrap();
        println!("Plane: {p1:?}, normal: {}", p1.normal());
        let p2 = Plane3d::from_points(
            point_3d(1.0, -1.0, 0.0),
            point_3d(3.0, -1.0, 2.0),
            point_3d(3.0, -1.0, -2.0),
        )
        .unwrap();
        println!("Plane: {p2:?}, normal: {}", p2.normal());
        assert_eq!(p1, p2);
        let d = p1.point_distance(&point_3d(0.0, 0.0, 0.0));
        assert_eq!(d, 1.0);
    }
    #[test]
    fn test_invalid_plane() {
        let plane = Plane3d::from_points(
            point_3d(0.0, 0.0, 0.0),
            point_3d(1.0, 0.0, 0.0),
            point_3d(2.0, 0.0, 0.0),
        );
        assert_eq!(Err(InvalidPlane {}), plane);
    }
    #[test]
    fn test_plane_intersect() {
        let p1 = Plane3d::from_points(
            point_3d(-1.0, -1.0, 0.0),
            point_3d(1.0, -1.0, 1.0),
            point_3d(1.0, -1.0, -1.0),
        )
        .unwrap();
        let p2 = Plane3d::from_points(
            point_3d(-1.0, -1.0, 0.0),
            point_3d(-1.0, 1.0, 1.0),
            point_3d(-1.0, 1.0, -1.0),
        )
        .unwrap();
        assert_eq!(
            PlaneCutRelationship::Line(StaticLine3d::new(
                point_3d(-1.0, -1.0, 0.0),
                point_3d(0.0, 0.0, 1.0)
            )),
            p1.relationship(&p2)
        );
    }
    #[test]
    fn test_plane_line_intersect() {
        let p = Plane3d::from_points(
            point_3d(1.0, 4.0, 0.0),
            point_3d(3.0, 2.0, 3.0),
            point_3d(1.0, 4.0, 3.0),
        )
        .unwrap();
        let l = StaticLine3d::from_points(point_3d(1.0, 2.0, 1.0), point_3d(3.0, 4.0, 3.0));
        let i = p.intersect_line_at(&l);
        assert_relative_eq!(i.0, 0.5);
        let point = l.point_at(i);
        assert_relative_eq!(point.x.0, 2.0);
        assert_relative_eq!(point.y.0, 3.0);
        assert_relative_eq!(point.z.0, 2.0);
    }
}
