use crate::geometry3d::line::static_line::PointLine3d;
use crate::geometry3d::line::Line3d;
use crate::geometry3d::point::Point3d;
use crate::geometry3d::triangles::indexed_point::IndexedPoint;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ReferencedLine<'a, P: Point3d> {
    p1: &'a P,
    p2: &'a P,
}

/*
impl<'a, P: Point3d> ReferencedLine<'a, IndexedPoint<'a, P>> {
pub fn normal(self) -> (Self, bool) {
    if self.p1.idx() < self.p2.idx() {
        (self, false)
    } else {
        (
            Self {
                p1: self.p2,
                p2: self.p1,
            },
            true,
        )
    }
}
}
 */

impl<'a, P: Point3d> Line3d<P> for ReferencedLine<'a, P> {
    fn p1(&self) -> P {
        self.p1.clone()
    }

    fn p2(&self) -> P {
        self.p2.clone()
    }
}

impl<'a, P: Point3d> ReferencedLine<'a, P> {
    pub fn new(p1: &'a P, p2: &'a P) -> Self {
        Self { p1, p2 }
    }
    pub fn deref(&self) -> PointLine3d<P> {
        PointLine3d::new(self.p1.clone(), self.p2.clone())
    }
    pub fn reverse(&self) -> ReferencedLine<'a, P> {
        Self {
            p1: self.p2,
            p2: self.p1,
        }
    }
}
