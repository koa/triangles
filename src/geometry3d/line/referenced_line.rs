use crate::geometry3d::line::Line3d;
use crate::geometry3d::point::Point3d;

pub struct ReferencedLine<'a, P: Point3d> {
    p1: &'a P,
    p2: &'a P,
}

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
}
