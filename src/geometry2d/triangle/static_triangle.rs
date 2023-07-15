use std::fmt::{Debug, Formatter};

use crate::geometry2d::{
    point::Point2d,
    polygon::{AnyPolygon, Polygon2d},
    triangle::{Triangle2d, TrianglePointIterator},
};

#[derive(Clone, PartialEq, Copy, Eq)]
pub struct StaticTriangle2d {
    p1: Point2d,
    p2: Point2d,
    p3: Point2d,
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
    pub fn new(p1: Point2d, p2: Point2d, p3: Point2d) -> Self {
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

    fn get_point(&self, idx: usize) -> Option<&'_ Point2d> {
        match idx {
            0 => Some(&self.p1),
            1 => Some(&self.p2),
            2 => Some(&self.p3),
            _ => None,
        }
    }
}

impl Triangle2d for StaticTriangle2d {
    fn p1(&self) -> &Point2d {
        &self.p1
    }

    fn p2(&self) -> &Point2d {
        &self.p2
    }

    fn p3(&self) -> &Point2d {
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
