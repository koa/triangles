use std::fmt::{Debug, Formatter};

use triangulate::Polygon;

use crate::geometry2d::triangle::{TriangleCornerIterator, TriangleCornerPoint};
use crate::geometry2d::{
    point::StaticPoint2d,
    polygon::{AnyPolygon, Polygon2d},
    triangle::{Triangle2d, TrianglePointIterator},
};

#[derive(Clone, PartialEq, Copy, Eq)]
pub struct StaticTriangle2d {
    p1: StaticPoint2d,
    p2: StaticPoint2d,
    p3: StaticPoint2d,
}

impl<'p> Polygon<'p> for StaticTriangle2d {
    type Vertex = StaticPoint2d;
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

impl Debug for StaticTriangle2d {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "T2d({:?},{:?},{:?})",
            self.p1, self.p2, self.p3
        ))
    }
}

impl StaticTriangle2d {
    pub fn new(p1: StaticPoint2d, p2: StaticPoint2d, p3: StaticPoint2d) -> Self {
        Self { p1, p2, p3 }
    }
}

impl Polygon2d for StaticTriangle2d {
    type PointIter<'a> = TrianglePointIterator<'a, StaticTriangle2d>;

    fn points(&self) -> Self::PointIter<'_> {
        TrianglePointIterator {
            triangle: self,
            state: Default::default(),
        }
    }

    fn point_count(&self) -> usize {
        3
    }

    fn to_any_polygon(self) -> AnyPolygon {
        AnyPolygon::StaticTrianglePolygon(self)
    }

    fn get_point(&self, idx: usize) -> Option<&'_ StaticPoint2d> {
        match idx {
            0 => Some(&self.p1),
            1 => Some(&self.p2),
            2 => Some(&self.p3),
            _ => None,
        }
    }
}

impl Triangle2d for StaticTriangle2d {
    fn p1(&self) -> &StaticPoint2d {
        &self.p1
    }

    fn p2(&self) -> &StaticPoint2d {
        &self.p2
    }

    fn p3(&self) -> &StaticPoint2d {
        &self.p3
    }

    fn reverse(&self) -> Self {
        Self {
            p1: self.p1,
            p2: self.p3,
            p3: self.p2,
        }
    }
}
