use std::fmt::Debug;

use log::info;
use triangulate::Vertex;
use vek::{Mat4, Vec3, Vec4};

use crate::geometry2d::triangle::TrianglePointIterator;
use crate::geometry3d::plane::Plane3d;
use crate::geometry3d::point::{point_3d, Point3d};
use crate::prelude::{Point2d, Polygon2d, StaticPoint2d, Triangle2d, Triangle3d};
use crate::primitives::Number;

pub struct PlaneProjection {
    projection: Mat4<Number>,
}

impl PlaneProjection {
    pub fn new(plane: &Plane3d) -> Self {
        let normal = plane.normal();
        let target = point_3d(0.0, 0.0, 1.0);
        let projection = Mat4::<Number>::rotation_from_to_3d(normal, target);
        Self { projection }
    }

    pub fn projection(&self) -> Mat4<Number> {
        self.projection
    }

    pub fn project_point<'a, Pt: Point3d>(&self, p: &'a Pt) -> ProjectedPoint2d<'a, Pt> {
        ProjectedPoint2d::project_point(p, &self.projection)
    }
    pub fn project_triangle<'a, Pt: Point3d, T: Triangle3d<Pt> + Debug + Clone>(
        &self,
        triangle: &'a T,
    ) -> ProjectedTriangle<'a, Pt, T> {
        ProjectedTriangle::project_triangle(triangle, &self.projection)
    }
}
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct ProjectedPoint2d<'a, P: Point3d> {
    origin: &'a P,
    x: Number,
    y: Number,
}

impl<'a, P: Point3d> ProjectedPoint2d<'a, P> {
    pub fn project_point(p: &'a P, q: &Mat4<Number>) -> Self {
        let v4: Vec3<f64> = p.coordinates().map(|v| v.0);

        let Vec4 { x, y, z, w } = q.map(|v| v.0) * Into::<Vec4<f64>>::into(v4);
        //info!("z: {z}, w: {w}");
        Self {
            origin: p,
            x: x.into(),
            y: y.into(),
        }
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
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct ProjectedTriangle<'a, P: Point3d, T: Triangle3d<P> + Debug + Clone> {
    origin: &'a T,
    p1: ProjectedPoint2d<'a, P>,
    p2: ProjectedPoint2d<'a, P>,
    p3: ProjectedPoint2d<'a, P>,
}

impl<'a, P: Point3d, T: Triangle3d<P> + Debug + Clone> Polygon2d<ProjectedPoint2d<'a, P>>
    for ProjectedTriangle<'a, P, T>
{
    type PointIter<'b>    = TrianglePointIterator<'b, ProjectedTriangle<'a, P, T>, ProjectedPoint2d<'a, P>> where
      Self: 'b,
      ProjectedPoint2d<'a, P>: 'b;

    fn points(&self) -> Self::PointIter<'_> {
        TrianglePointIterator::new(self)
    }

    fn point_count(&self) -> usize {
        3
    }

    fn get_point(&self, idx: usize) -> Option<&ProjectedPoint2d<'a, P>> {
        match idx {
            0 => Some(&self.p1),
            1 => Some(&self.p2),
            2 => Some(&self.p3),
            _ => None,
        }
    }
}

impl<'a, P: Point3d, T: Triangle3d<P> + Debug + Clone> Triangle2d<ProjectedPoint2d<'a, P>>
    for ProjectedTriangle<'a, P, T>
{
    fn p1(&self) -> &ProjectedPoint2d<'a, P> {
        &self.p1
    }

    fn p2(&self) -> &ProjectedPoint2d<'a, P> {
        &self.p2
    }

    fn p3(&self) -> &ProjectedPoint2d<'a, P> {
        &self.p3
    }
}

impl<'a, P: Point3d, T: Triangle3d<P> + Debug + Clone> ProjectedTriangle<'a, P, T> {
    pub fn project_triangle(origin: &'a T, q: &Mat4<Number>) -> Self {
        let [p1, p2, p3] = origin.points();
        Self {
            origin,
            p1: ProjectedPoint2d::project_point(p1, q),
            p2: ProjectedPoint2d::project_point(p2, q),
            p3: ProjectedPoint2d::project_point(p3, q),
        }
    }
}
