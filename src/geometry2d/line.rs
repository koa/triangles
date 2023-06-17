use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};

use num_traits::Zero;
use ordered_float::OrderedFloat;

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
    fn y_cross_side(&self, p: &Point2d) -> HitSide {
        let Point2d { x, y } = *p;
        let Point2d { x: x1, y: y1 } = *self.p1();
        let Point2d { x: x2, y: y2 } = *self.p2();
        if Number::min(y1, y2) > y || Number::max(y1, y2) < y {
            HitSide::None
        } else if y1 == y2 {
            if Number::min(x1, x2) > x || Number::max(x1, x2) < x {
                HitSide::None
            } else {
                HitSide::OnLine
            }
        } else if y == y1 {
            same_height_as_endpoint_cases(x.cmp(&x1), y.cmp(&y2))
        } else if y == y2 {
            same_height_as_endpoint_cases(x.cmp(&x2), y.cmp(&y1))
        } else {
            let x_cross = x1 + ((x2 - x1) / (y2 - y1)) * (y - y1);
            match x.cmp(&x_cross) {
                Ordering::Less => HitSide::Left,
                Ordering::Equal => HitSide::OnLine,
                Ordering::Greater => HitSide::Right,
            }
        }
    }

    /*fn debug_fmt<L: Line2d>(line: &L, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({:?})-({:?})", line.p1(), line.p2()))
    }*/
}
#[inline]
fn same_height_as_endpoint_cases(own_x_compare: Ordering, other_y_compare: Ordering) -> HitSide {
    match (own_x_compare, other_y_compare) {
        (_, Ordering::Equal) => panic!("Math Error"),
        (Ordering::Equal, _) => HitSide::OnLine,
        (Ordering::Less, Ordering::Greater) => HitSide::LeftTop,
        (Ordering::Less, Ordering::Less) => HitSide::LeftBottom,
        (Ordering::Greater, Ordering::Greater) => HitSide::RightTop,
        (Ordering::Greater, Ordering::Less) => HitSide::RightBottom,
    }
}
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum HitSide {
    None,
    OnLine,
    Left,
    Right,
    LeftTop,
    LeftBottom,
    RightTop,
    RightBottom,
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

#[cfg(test)]
mod test {
    use crate::geometry2d::line::{HitSide, Line2d, StaticLine2d};

    #[test]
    fn test_point_line_positive() {
        let line = StaticLine2d::new((1.0, 1.0).into(), (2.0, 2.0).into());

        assert_eq!(HitSide::None, line.y_cross_side(&(0.0, 0.0).into()));
        assert_eq!(HitSide::None, line.y_cross_side(&(0.0, 3.0).into()));

        assert_eq!(HitSide::Left, line.y_cross_side(&(0.0, 1.5).into()));
        assert_eq!(HitSide::OnLine, line.y_cross_side(&(1.5, 1.5).into()));
        assert_eq!(HitSide::Right, line.y_cross_side(&(2.0, 1.5).into()));

        assert_eq!(HitSide::LeftBottom, line.y_cross_side(&(0.0, 1.0).into()));
        assert_eq!(HitSide::OnLine, line.y_cross_side(&(1.0, 1.0).into()));
        assert_eq!(HitSide::RightBottom, line.y_cross_side(&(2.0, 1.0).into()));

        assert_eq!(HitSide::LeftTop, line.y_cross_side(&(1.0, 2.0).into()));
        assert_eq!(HitSide::OnLine, line.y_cross_side(&(2.0, 2.0).into()));
        assert_eq!(HitSide::RightTop, line.y_cross_side(&(3.0, 2.0).into()));
    }
    #[test]
    fn test_point_line_negative() {
        let line = StaticLine2d::new((2.0, 1.0).into(), (1.0, 2.0).into());

        assert_eq!(HitSide::None, line.y_cross_side(&(0.0, 0.0).into()));
        assert_eq!(HitSide::None, line.y_cross_side(&(0.0, 3.0).into()));

        assert_eq!(HitSide::Left, line.y_cross_side(&(0.0, 1.5).into()));
        assert_eq!(HitSide::OnLine, line.y_cross_side(&(1.5, 1.5).into()));
        assert_eq!(HitSide::Right, line.y_cross_side(&(2.0, 1.5).into()));

        assert_eq!(HitSide::LeftBottom, line.y_cross_side(&(0.0, 1.0).into()));
        assert_eq!(HitSide::OnLine, line.y_cross_side(&(2.0, 1.0).into()));
        assert_eq!(HitSide::RightBottom, line.y_cross_side(&(3.0, 1.0).into()));
    }
    #[test]
    fn test_point_line_horizontal() {
        let line = StaticLine2d::new((2.0, 1.0).into(), (1.0, 1.0).into());

        assert_eq!(HitSide::None, line.y_cross_side(&(0.0, 0.0).into()));
        assert_eq!(HitSide::None, line.y_cross_side(&(0.0, 2.0).into()));

        assert_eq!(HitSide::None, line.y_cross_side(&(0.0, 1.0).into()));
        assert_eq!(HitSide::OnLine, line.y_cross_side(&(1.5, 1.0).into()));
        assert_eq!(HitSide::None, line.y_cross_side(&(3.0, 1.0).into()));
    }
    #[test]
    fn test_point_line_vertical() {
        let line = StaticLine2d::new((1.0, 0.0).into(), (1.0, 1.0).into());
        assert_eq!(HitSide::RightBottom, line.y_cross_side(&(1.5, 0.0).into()));
    }
}
