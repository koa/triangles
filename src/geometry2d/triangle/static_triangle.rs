use std::fmt::{Debug, Formatter};

use triangulate::Polygon;

use crate::geometry2d::triangle::{TriangleCornerIterator, TriangleCornerPoint};
use crate::geometry2d::{
    point::StaticPoint2d,
    polygon::{AnyPolygon, Polygon2d},
    triangle::{Triangle2d, TrianglePointIterator},
};
use crate::prelude::Point2d;

#[derive(Clone, PartialEq, Copy, Eq)]
pub struct StaticTriangle2d<Pt: Point2d> {
    pub p1: Pt,
    pub p2: Pt,
    pub p3: Pt,
}

impl<Pt: Point2d> StaticTriangle2d<Pt> {
    pub fn coordinates_triangle(&self) -> StaticTriangle2d<StaticPoint2d> {
        StaticTriangle2d {
            p1: self.p1.coordinates(),
            p2: self.p2.coordinates(),
            p3: self.p3.coordinates(),
        }
    }
}

impl<'p, Pt: Point2d + 'p + triangulate::Vertex> Polygon<'p> for StaticTriangle2d<Pt> {
    type Vertex = Pt;
    type Index = TriangleCornerPoint;
    type Iter<'i>    = TriangleCornerIterator  where Self: 'i, Self::Vertex: 'i, 'p: 'i;

    fn vertex_count(&self) -> usize {
        3
    }

    fn iter_indices<'i>(&'i self) -> Self::Iter<'i>
    where
        Self: 'i,
        Self::Vertex: 'i,
        'p: 'i,
    {
        Default::default()
    }

    fn get_vertex(&self, index: Self::Index) -> &Self::Vertex {
        self.point(index)
    }
}

impl<Pt: Point2d> Debug for StaticTriangle2d<Pt> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "T2d({:?},{:?},{:?})",
            self.p1, self.p2, self.p3
        ))
    }
}

impl<Pt: Point2d> StaticTriangle2d<Pt> {
    pub fn new(p1: Pt, p2: Pt, p3: Pt) -> Self {
        Self { p1, p2, p3 }
    }
}

impl<Pt: Point2d> Polygon2d<Pt> for StaticTriangle2d<Pt> {
    type PointIter<'a> = TrianglePointIterator<'a, StaticTriangle2d<Pt>, Pt> where Pt: 'a;

    fn points(&self) -> Self::PointIter<'_> {
        TrianglePointIterator {
            phantom: Default::default(),
            triangle: self,
            state: Default::default(),
        }
    }

    fn point_count(&self) -> usize {
        3
    }

    fn to_any_polygon(self) -> AnyPolygon<Pt> {
        AnyPolygon::StaticTrianglePolygon(self)
    }

    fn get_point(&self, idx: usize) -> Option<&'_ Pt> {
        match idx {
            0 => Some(&self.p1),
            1 => Some(&self.p2),
            2 => Some(&self.p3),
            _ => None,
        }
    }
}

impl<Pt: Point2d> Triangle2d<Pt> for StaticTriangle2d<Pt> {
    fn p1(&self) -> &Pt {
        &self.p1
    }

    fn p2(&self) -> &Pt {
        &self.p2
    }

    fn p3(&self) -> &Pt {
        &self.p3
    }

    /*
    fn reverse(&self) -> Self {
        Self {
            p1: self.p1.clone(),
            p2: self.p3.clone(),
            p3: self.p2.clone(),
        }
    }*/
}
