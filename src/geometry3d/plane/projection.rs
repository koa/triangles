use triangulate::Vertex;
use vek::{Quaternion, Vec3};

use crate::geometry3d::plane::Plane3d;
use crate::geometry3d::point::{point_3d, Point3d};
use crate::prelude::{Point2d, StaticPoint2d};
use crate::primitives::Number;

pub struct PlaneProjection {
    plane: Plane3d,
    projection: Quaternion<Number>,
}

impl PlaneProjection {
    pub fn new(plane: Plane3d) -> Self {
        let projection =
            Quaternion::<Number>::rotation_from_to_3d(plane.normal(), point_3d(0.0, 0.0, 1.0));
        Self { plane, projection }
    }

    pub fn plane(&self) -> &Plane3d {
        &self.plane
    }
    pub fn projection(&self) -> Quaternion<Number> {
        self.projection
    }

    pub fn project<'a, Pt: Point3d>(&self, p: &'a Pt) -> ProjectedPoint2d<'a, Pt> {
        ProjectedPoint2d::project_point(p, &self.projection)
    }
}
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct ProjectedPoint2d<'a, P: Point3d> {
    origin: &'a P,
    x: Number,
    y: Number,
}

impl<'a, P: Point3d> ProjectedPoint2d<'a, P> {
    pub fn project_point(p: &'a P, q: &Quaternion<Number>) -> Self {
        let Vec3 { x, y, z: _ } = p.coordinates() * *q;
        Self { origin: p, x, y }
    }
}

impl<'a, P: Point3d> Vertex for ProjectedPoint2d<'a, P> {
    type Coordinate = Number;

    fn x(&self) -> Self::Coordinate {
        self.x
    }

    fn y(&self) -> Self::Coordinate {
        self.y
    }
}

impl<'a, P: Point3d> Point2d for ProjectedPoint2d<'a, P> {
    fn coordinates(&self) -> StaticPoint2d {
        StaticPoint2d {
            x: self.x,
            y: self.y,
        }
    }
}
