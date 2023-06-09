use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};

use num_traits::Zero;
use ordered_float::OrderedFloat;
use triangulate::Vertex;

use crate::geometry2d::point::Point2d;
use crate::geometry2d::vector::Vector2d;
use crate::primitives::Number;

pub trait Line2d: Sized + Debug {
    fn p1(&self) -> &Point2d;
    fn p2(&self) -> &Point2d;
    fn direction(&self) -> Vector2d {
        *self.p2() - *self.p1()
    }
    fn equals<L2: Line2d>(&self, other: &L2) -> bool {
        self.p1() == other.p1() && self.p2() == other.p2()
    }
    fn side_of_pt(&self, pt: &Point2d) -> SideOfLine {
        let p1 = self.p1();
        let p2 = self.p2();
        let r = (pt.x() - p2.x()) * (p1.y() - p2.y()) - (p1.x() - p2.x()) * (pt.y() - p2.y());
        match r.cmp(&OrderedFloat::zero()) {
            Ordering::Less => SideOfLine::Right,
            Ordering::Equal => SideOfLine::Hit,
            Ordering::Greater => SideOfLine::Left,
        }
    }
    fn pt_along(&self, n: Number) -> Point2d {
        *self.p1() + (*self.p2() - *self.p1()) * n
    }

    fn intersect<L: Line2d>(&self, other: &L) -> LineIntersection {
        let p1 = self.p1();
        let p2 = other.p1();
        let v1 = self.direction();
        let v2 = other.direction();
        let div = (v1.x()) * (v2.y()) - (v1.y()) * (v2.x());
        if div != 0.0 {
            let p_diff = *p1 - *p2;
            let ua = (v2.x() * p_diff.y() - v2.y() * p_diff.x()) / div;
            let ub = (v1.x() * p_diff.y() - v1.y() * p_diff.x()) / div;
            if (0.0..=1.0).contains(&ua.into()) && (0.0..=1.0).contains(&ub.into()) {
                LineIntersection::Point {
                    my_pos: ua,
                    other_pos: ub,
                }
            } else {
                LineIntersection::None
            }
        } else {
            LineIntersection::None
        }
    }

    /*fn debug_fmt<L: Line2d>(line: &L, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({:?})-({:?})", line.p1(), line.p2()))
    }*/
}

pub enum LineIntersection {
    Point { my_pos: Number, other_pos: Number },
    None,
}

impl Into<Option<(Number, Number)>> for LineIntersection {
    fn into(self) -> Option<(Number, Number)> {
        match self {
            LineIntersection::Point { my_pos, other_pos } => Some((my_pos, other_pos)),
            LineIntersection::None => None,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SideOfLine {
    Left,
    Right,
    Hit,
}

pub struct StaticLine2d {
    p1: Point2d,
    p2: Point2d,
}

impl StaticLine2d {
    pub fn new(p1: Point2d, p2: Point2d) -> StaticLine2d {
        StaticLine2d { p1, p2 }
    }
}

impl PartialEq for StaticLine2d {
    fn eq(&self, other: &Self) -> bool {
        self.p1 == other.p1 && self.p2 == other.p2
    }
}

impl Debug for StaticLine2d {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({:?})-({:?})", self.p1, self.p2))
    }
}

impl Line2d for StaticLine2d {
    fn p1(&self) -> &Point2d {
        &self.p1
    }

    fn p2(&self) -> &Point2d {
        &self.p2
    }
}

pub struct ReferenceLine2d<'a> {
    p1: &'a Point2d,
    p2: &'a Point2d,
}

impl<'a> Debug for ReferenceLine2d<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({:?})-({:?})", self.p1(), self.p2()))
    }
}

impl<'a> Line2d for ReferenceLine2d<'a> {
    fn p1(&self) -> &Point2d {
        self.p1
    }

    fn p2(&self) -> &Point2d {
        self.p2
    }
}

impl<'a> ReferenceLine2d<'a> {
    pub fn new(p1: &'a Point2d, p2: &'a Point2d) -> Self {
        Self { p1, p2 }
    }
}

impl Line2d for (&Point2d, &Point2d) {
    fn p1(&self) -> &Point2d {
        self.0
    }

    fn p2(&self) -> &Point2d {
        self.1
    }
}
