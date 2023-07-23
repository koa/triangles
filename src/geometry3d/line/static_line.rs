use vek::Vec3;

use crate::geometry3d::line::Line3d;
use crate::geometry3d::point::Point3d;
use crate::primitives::Number;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct StaticLine3d {
    p: Vec3<Number>,
    dir: Vec3<Number>,
}

impl StaticLine3d {
    pub fn new(p: Vec3<Number>, dir: Vec3<Number>) -> Self {
        Self { p, dir }
    }
}

impl Line3d<Vec3<Number>> for StaticLine3d {
    fn p1(&self) -> Vec3<Number> {
        self.p
    }

    fn p2(&self) -> Vec3<Number> {
        self.p + self.dir
    }

    fn direction(&self) -> Vec3<Number> {
        self.dir
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct PointLine3d<P: Point3d> {
    p1: P,
    p2: P,
}

impl<P: Point3d> Line3d<P> for PointLine3d<P> {
    fn p1(&self) -> P {
        self.p1.clone()
    }

    fn p2(&self) -> P {
        self.p2.clone()
    }
}

impl<P: Point3d> PointLine3d<P> {
    pub fn new(p1: P, p2: P) -> Self {
        Self { p1, p2 }
    }
}
