use std::ops::Add;

use ordered_float::OrderedFloat;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

use crate::geometry2d::line::{Line2d, SideOfLine, StaticLine2d};
use crate::geometry2d::point::{BoundingBoxSvg, Point2d};
use crate::geometry2d::polygon::{AnyPolygon, Polygon2d, PolygonPath};
use crate::geometry2d::triangle::{StaticTriangle2d, Triangle2d};

struct Figure {
    path: PathWay,
    fill: Option<String>,
    stroke: Option<String>,
    width: Option<u8>,
}
enum PathWay {
    Polygon(AnyPolygon),
    PointList(Vec<Point2d>),
}

impl PathWay {
    fn expand_bbox(&self, bbox: &mut BoundingBoxSvg) {
        match self {
            PathWay::Polygon(p) => {
                for p in p.points() {
                    *bbox += *p;
                }
            }
            PathWay::PointList(l) => {
                for p in l {
                    *bbox += *p;
                }
            }
        }
    }
}

impl<P: Polygon2d> From<P> for PathWay {
    fn from(value: P) -> Self {
        Self::Polygon(value.to_any_polygon())
    }
}

impl Figure {
    fn from_polygon<P: Polygon2d, F: ToString, S: ToString, W: Into<u8>>(
        polygon: P,
        fill: F,
        stroke: S,
        width: W,
    ) -> Self {
        Self {
            path: PathWay::Polygon(polygon.to_any_polygon()),
            fill: Some(fill.to_string()),
            stroke: Some(stroke.to_string()),
            width: Some(width.into()),
        }
    }
    fn from_points<F: ToString, S: ToString, W: Into<u8>>(
        points: Vec<Point2d>,
        fill: F,
        stroke: S,
        width: W,
    ) -> Self {
        Self {
            path: PathWay::PointList(points),
            fill: Some(fill.to_string()),
            stroke: Some(stroke.to_string()),
            width: Some(width.into()),
        }
    }
}

#[derive(Default)]
struct DisplayList {
    entries: Vec<Figure>,
}

impl<T: Polygon2d> From<T> for Figure {
    fn from(value: T) -> Self {
        Figure {
            path: PathWay::Polygon(value.to_any_polygon()),
            fill: None,
            stroke: None,
            width: None,
        }
    }
}

impl DisplayList {
    fn append_figure(&mut self, figure: Figure) {
        self.entries.push(figure);
    }
    fn plot<T: AsRef<std::path::Path>>(&self, path: T) -> std::io::Result<()> {
        let mut bbox: BoundingBoxSvg = Default::default();
        for f in &self.entries {
            f.path.expand_bbox(&mut bbox);
        }
        let mut svg = bbox.plot_coordinates(Document::new().set("viewBox", bbox));
        for f in &self.entries {
            let fill = f.fill.as_deref().unwrap_or("none");
            let color = f.stroke.as_deref().unwrap_or("black");
            let width = f.width.unwrap_or(1);
            match &f.path {
                PathWay::Polygon(path) => {
                    if let Some(path) = create_path(&bbox, path, fill, color, width) {
                        svg = svg.add(path)
                    }
                }
                PathWay::PointList(points) => {
                    let mut iter = points.iter();
                    if let Some(start_pt) = iter.next() {
                        let mut data = Data::new().move_to(bbox.apply(start_pt));
                        for next_pt in iter {
                            data = data.line_to(bbox.apply(next_pt));
                        }
                        let path = Path::new()
                            .set("fill", fill)
                            .set("stroke", color)
                            .set("stroke-width", width)
                            .set("d", data);
                        svg = svg.add(path);
                    }
                }
            }
        }
        svg::save(path, &svg)
    }
}

fn create_path<P: Polygon2d>(
    bbox: &BoundingBoxSvg,
    polygon: &P,
    fill: &str,
    color: &str,
    width: u8,
) -> Option<Path> {
    polygon.plot(bbox).map(|data| {
        Path::new()
            .set("fill", fill)
            .set("stroke", color)
            .set("stroke-width", width)
            .set("d", data)
    })
}

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
    let big_triangle = StaticTriangle2d::new(
        (-100.0, -50.0).into(),
        (100.0, -50.0).into(),
        (0.0, 50.0).into(),
    );
    let small_triangle = StaticTriangle2d::new(
        (-50.0, 25.0).into(),
        (50.0, 25.0).into(),
        (0.0, -25.0).into(),
    );

    let cut_polygon = &small_triangle;
    let path = big_triangle.cut(cut_polygon);
    println!("Path: {path:?}");
    let mut list = DisplayList::default();
    match path {
        PolygonPath::Enclosed => {
            list.append_figure(Figure::from_polygon(cut_polygon.clone(), "none", "red", 2))
        }
        PolygonPath::CutSegments(segments) => {
            for segment in segments {
                let mut points = Vec::new();
                let start_cut = segment.start_cut();
                let end_cut = segment.end_cut();
                if let (Some(start_line), Some(end_line)) = (
                    cut_polygon.lines().nth(start_cut.start_pt_idx()),
                    cut_polygon.lines().nth(end_cut.start_pt_idx()),
                ) {
                    points.push(start_line.pt_along(start_cut.cut_pos()));
                    for p in cut_polygon.points_of_range(segment.copy_points()) {
                        points.push(*p);
                    }
                    points.push(end_line.pt_along(end_cut.cut_pos()));
                    list.append_figure(Figure::from_points(points, "none", "red", 2));
                }
            }
        }
        PolygonPath::None => {}
    }
    list.append_figure(big_triangle.into());
    list.append_figure(small_triangle.into());
    list.plot("target/triangle.svg").unwrap(); /*
                                               let bbox = big_triangle.bbox() + small_triangle.bbox();
                                               let mut svg = bbox.plot_coordinates(Document::new().set("viewBox", bbox));
                                               for triangle in [&big_triangle, &small_triangle] {
                                                   if let Some(path) = create_path(&bbox, triangle, "none", "black", 1) {
                                                       svg = svg.add(path)
                                                   }
                                               }
                                               svg::save("target/triangle.svg", &svg).unwrap();
                                               */
}
