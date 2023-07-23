use vek::Vec3;

use crate::geometry3d::line::Line3d;
use crate::primitives::Number;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct StaticLine {
    p: Vec3<Number>,
    dir: Vec3<Number>,
}

impl StaticLine {
    pub fn new(p: Vec3<Number>, dir: Vec3<Number>) -> Self {
        Self { p, dir }
    }
}

impl Line3d<Vec3<Number>> for StaticLine {
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
