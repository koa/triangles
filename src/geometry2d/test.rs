use ordered_float::OrderedFloat;

use crate::geometry2d::line::{Line2d, SideOfLine, StaticLine2d};
use crate::geometry2d::polygon::Polygon2d;
use crate::geometry2d::triangle::{StaticTriangle2d, Triangle2d};

#[test]
fn test_triangle_point_iterator() {
    let tr = StaticTriangle2d::new((2.0, 0.0).into(), (3.0, 0.0).into(), (2.0, 1.0).into());
    let mut iterator = tr.points();
    assert_eq!(3, iterator.len());
    assert_eq!(Some(&(2.0, 0.0).into()), iterator.next());
    assert_eq!(2, iterator.len());
    assert_eq!(Some(&(3.0, 0.0).into()), iterator.next());
    assert_eq!(1, iterator.len());
    assert_eq!(Some(&(2.0, 1.0).into()), iterator.next());
    assert_eq!(0, iterator.len());
    assert_eq!(None, iterator.next());
}

#[test]
fn test_area() {
    assert_eq!(
        OrderedFloat::from(0.5),
        StaticTriangle2d::new((2.0, 0.0).into(), (3.0, 0.0).into(), (2.0, 1.0).into()).area()
    );
    assert_eq!(
        OrderedFloat::from(-0.5),
        StaticTriangle2d::new((2.0, 0.0).into(), (2.0, 1.0).into(), (3.0, 0.0).into()).area()
    );
}

#[test]
fn test_triangle_line_iterator() {
    let tr = StaticTriangle2d::new((2.0, 0.0).into(), (3.0, 0.0).into(), (2.0, 1.0).into());
    let mut iterator = tr.lines();
    assert!(iterator
        .next()
        .map(|l1| l1.equals(&StaticLine2d::new((2.0, 0.0).into(), (3.0, 0.0).into())))
        .unwrap_or(false));
    assert!(iterator
        .next()
        .map(|l1| l1.equals(&StaticLine2d::new((3.0, 0.0).into(), (2.0, 1.0).into())))
        .unwrap_or(false));
    assert!(iterator
        .next()
        .map(|l1| l1.equals(&StaticLine2d::new((2.0, 1.0).into(), (2.0, 0.0).into())))
        .unwrap_or(false));
    assert!(iterator.next().is_none());
}

#[test]
fn test_side_of_line() {
    let l1 = StaticLine2d::new((0.0, 0.0).into(), (2.0, 2.0).into());
    assert_eq!(SideOfLine::Left, l1.side_of_pt(&(0.0, 1.0).into()));
    assert_eq!(SideOfLine::Hit, l1.side_of_pt(&(1.0, 1.0).into()));
    assert_eq!(SideOfLine::Right, l1.side_of_pt(&(1.0, 0.0).into()));

    let l2 = StaticLine2d::new((2.0, 2.0).into(), (0.0, 0.0).into());
    assert_eq!(SideOfLine::Right, l2.side_of_pt(&(0.0, 1.0).into()));
    assert_eq!(SideOfLine::Hit, l2.side_of_pt(&(1.0, 1.0).into()));
    assert_eq!(SideOfLine::Left, l2.side_of_pt(&(1.0, 0.0).into()));
}

#[test]
fn test_polygon_intersection() {
    let big_triangle =
        StaticTriangle2d::new((-10.0, -5.0).into(), (10.0, -5.0).into(), (0.0, 5.0).into());
    let small_triangle =
        StaticTriangle2d::new((-5.0, -2.5).into(), (5.0, -2.5).into(), (0.0, 2.5).into());
    let path = big_triangle.cut(&big_triangle);
    println!("Path: {path:?}");
}
