use num_traits::{One, Zero};
use vek::Vec3;

use crate::geometry3d::point::Point3d;
use crate::geometry3d::Vector3d;
use crate::primitives::Number;

pub mod static_line;

pub trait Line3d<P: Point3d> {
    fn p1(&self) -> P;
    fn p2(&self) -> P;
    fn direction(&self) -> Vec3<Number> {
        self.p2().coordinates() - self.p1().coordinates()
    }
    fn point_at(&self, along: Number) -> Vector3d {
        if along == Number::zero() {
            self.p1().coordinates()
        } else if along == Number::one() {
            self.p2().coordinates()
        } else {
            self.p1().coordinates() + self.direction() * along
        }
    }
}
