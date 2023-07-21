use ordered_float::OrderedFloat;
use svg::node::element::path::{Data, Parameters};
use svg::node::element::{Path, SVG};
use svg::node::Value;
use svg::Document;
use triangulate::{formats, ListFormat, Mappable, PolygonList};

use crate::geometry2d::line::{Line2d, SideOfLine, StaticLine2d};
use crate::geometry2d::point::StaticPoint2d;
use crate::geometry2d::polygon::cut::PolygonPath;
use crate::geometry2d::polygon::{AnyPolygon, Polygon2d};
use crate::geometry2d::triangle::Triangle2d;
use crate::prelude::{BoundingBox, BoundingBoxValues, Point2d, StaticTriangle2d};

struct Figure<Pt: Point2d> {
    path: PathWay<Pt>,
    fill: Option<String>,
    stroke: Option<String>,
    width: Option<u8>,
}
enum PathWay<Pt: Point2d> {
    Polygon(AnyPolygon<Pt>),
    PointList(Vec<Pt>),
}

impl<Pt: Point2d> PathWay<Pt> {
    fn expand_bbox(&self, bbox: &mut BoundingBox) {
        match self {
            PathWay::Polygon(p) => {
                for p in p.points() {
                    *bbox += p.coordinates();
                }
            }
            PathWay::PointList(l) => {
                for p in l {
                    *bbox += p.coordinates();
                }
            }
        }
    }
}

/*
impl<P, Pt> From<P> for PathWay<Pt>
where
    P: Polygon2d<Pt>,
{
    fn from(value: P) -> Self {
        Self::Polygon(value.to_any_polygon())
    }
}

 */

impl<Pt: Point2d> Figure<Pt> {
    fn from_polygon<P: Polygon2d<Pt>, F: ToString, S: ToString, W: Into<u8>>(
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
        points: Vec<Pt>,
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

struct DisplayList<Pt: Point2d> {
    entries: Vec<Figure<Pt>>,
}

impl<Pt: Point2d> Default for DisplayList<Pt> {
    fn default() -> Self {
        Self { entries: vec![] }
    }
}

pub fn project<Pt: Point2d>(bbox: &BoundingBox, point: &Pt) -> Parameters {
    let StaticPoint2d { x, y } = point.coordinates();
    match bbox {
        BoundingBox::Empty => Parameters::from((x.0 as f32, y.0 as f32)),
        BoundingBox::Box(BoundingBoxValues { min_x, min_y, .. }) => {
            Parameters::from((x.0 as f32 - min_x.0 as f32, y.0 as f32 - min_y.0 as f32))
        }
    }
}

pub fn plot_coordinates(bbox: &BoundingBox, svg: SVG) -> SVG {
    if let BoundingBox::Box(BoundingBoxValues {
        min_x,
        min_y,
        max_x,
        max_y,
    }) = *bbox
    {
        let svg = if *min_x <= 0.0 && *max_x >= 0.0 {
            let min_pt = project::<StaticPoint2d>(bbox, &(0.0, *min_y as f64).into());
            let max_pt = project::<StaticPoint2d>(bbox, &(0.0, *max_y as f64).into());
            svg.add(
                Path::new()
                    .set("fill", "none")
                    .set("stroke", "blue")
                    .set("stroke-width", 0.1)
                    .set("d", Data::new().move_to(min_pt).line_to(max_pt)),
            )
        } else {
            svg
        };
        if *min_y <= 0.0 && *max_y >= 0.0 {
            let min_pt = project::<StaticPoint2d>(bbox, &(*min_x as f64, 0.0).into());
            let max_pt = project::<StaticPoint2d>(bbox, &(*max_x as f64, 0.0).into());
            svg.add(
                Path::new()
                    .set("fill", "none")
                    .set("stroke", "blue")
                    .set("stroke-width", 0.1)
                    .set("d", Data::new().move_to(min_pt).line_to(max_pt)),
            )
        } else {
            svg
        }
    } else {
        svg
    }
}
fn bbox2value(bbox: &BoundingBox) -> Value {
    match bbox {
        BoundingBox::Empty => "".into(),
        BoundingBox::Box(BoundingBoxValues {
            min_x,
            min_y,
            max_x,
            max_y,
        }) => {
            let span_x = *max_x - *min_x;
            let span_y = *max_y - *min_y;
            (0, 0, span_x.0, span_y.0).into()
        }
    }
}

impl<Pt: Point2d> DisplayList<Pt> {
    fn append_figure(&mut self, figure: Figure<Pt>) {
        self.entries.push(figure);
    }
    fn plot<T: AsRef<std::path::Path>>(&self, path: T) -> std::io::Result<()> {
        let mut bbox: BoundingBox = Default::default();
        for f in &self.entries {
            f.path.expand_bbox(&mut bbox);
        }
        let mut svg = plot_coordinates(&bbox, Document::new().set("viewBox", bbox2value(&bbox)));
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
                        let mut data = Data::new().move_to(project(&bbox, start_pt));
                        for next_pt in iter {
                            data = data.line_to(project(&bbox, next_pt));
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
fn plot<P: Polygon2d<Pt>, Pt: Point2d>(polygon: &P, bbox: &BoundingBox) -> Option<Data> {
    let mut iter = polygon.points();
    if let Some(start_pt) = iter.next() {
        let mut data = Data::new().move_to(project(&bbox, start_pt));
        for next_pt in iter {
            data = data.line_to(project(&bbox, next_pt));
        }
        Some(data.close())
    } else {
        None
    }
}

fn create_path<P: Polygon2d<Pt>, Pt: Point2d>(
    bbox: &BoundingBox,
    polygon: &P,
    fill: &str,
    color: &str,
    width: u8,
) -> Option<Path> {
    plot(polygon, bbox).map(|data| {
        Path::new()
            .set("fill", fill)
            .set("stroke", color)
            .set("stroke-width", width)
            .set("d", data)
    })
}

#[test]
fn test_triangle_point_iterator() {
    let tr = StaticTriangle2d::<StaticPoint2d>::new(
        (2.0, 0.0).into(),
        (3.0, 0.0).into(),
        (2.0, 1.0).into(),
    );
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
        StaticTriangle2d::<StaticPoint2d>::new(
            (2.0, 0.0).into(),
            (3.0, 0.0).into(),
            (2.0, 1.0).into()
        )
        .area()
    );
    assert_eq!(
        OrderedFloat::from(-0.5),
        StaticTriangle2d::<StaticPoint2d>::new(
            (2.0, 0.0).into(),
            (2.0, 1.0).into(),
            (3.0, 0.0).into()
        )
        .area()
    );
}

#[test]
fn test_triangle_line_iterator() {
    let tr = StaticTriangle2d::<StaticPoint2d>::new(
        (2.0, 0.0).into(),
        (3.0, 0.0).into(),
        (2.0, 1.0).into(),
    );
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
    let l1 = StaticLine2d::<StaticPoint2d>::new((0.0, 0.0).into(), (2.0, 2.0).into());
    assert_eq!(SideOfLine::Left, l1.side_of_pt(&(0.0, 1.0).into()));
    assert_eq!(SideOfLine::Hit, l1.side_of_pt(&(1.0, 1.0).into()));
    assert_eq!(SideOfLine::Right, l1.side_of_pt(&(1.0, 0.0).into()));

    let l2 = StaticLine2d::<StaticPoint2d>::new((2.0, 2.0).into(), (0.0, 0.0).into());
    assert_eq!(SideOfLine::Right, l2.side_of_pt(&(0.0, 1.0).into()));
    assert_eq!(SideOfLine::Hit, l2.side_of_pt(&(1.0, 1.0).into()));
    assert_eq!(SideOfLine::Left, l2.side_of_pt(&(1.0, 0.0).into()));
}

#[test]
fn test_triangle_error() {
    let big_triangle = StaticTriangle2d::<StaticPoint2d>::new(
        (-100.0, 0.0).into(),
        (100.0, 0.0).into(),
        (0.0, 100.0).into(),
    );
    let small_triangle = StaticTriangle2d::<StaticPoint2d>::new(
        (-50.0, 25.0).into(),
        (00.0, -25.0).into(),
        (50.0, 25.0).into(),
    );
    let path = big_triangle.cut(&small_triangle);
    match &path {
        PolygonPath::Enclosed => {}
        PolygonPath::CutSegments(segments) => {
            for segment in segments {
                let mut points = Vec::new();
                let start_cut = segment.start_cut();
                let end_cut = segment.end_cut();
                if let (Some(start_line), Some(end_line)) = (
                    small_triangle.lines().nth(start_cut.start_pt_idx()),
                    small_triangle.lines().nth(end_cut.start_pt_idx()),
                ) {
                    points.push(start_line.pt_along(start_cut.polygon_pos()));
                    for p in small_triangle.points_of_range(segment.copy_points()) {
                        points.push(*p);
                    }
                    points.push(end_line.pt_along(end_cut.polygon_pos()));
                    println!("Points: {points:?}")
                }
            }
        }
        PolygonPath::None => {}
    }
}

#[test]
fn test_polygon_intersection() {
    let big_triangle = StaticTriangle2d::new(
        (-100.0, -50.0).into(),
        (100.0, -50.0).into(),
        (0.0, 50.0).into(),
    );
    let small_triangle = StaticTriangle2d::<StaticPoint2d>::new(
        (-50.0, 25.0).into(),
        (0.0, -25.0).into(),
        (50.0, 25.0).into(),
    );
    let small_triangle =
        StaticTriangle2d::new((0.0, 50.0).into(), (0.0, 25.0).into(), (-50.0, 50.0).into());
    let cut_polygon = &small_triangle;
    let path = big_triangle.cut(cut_polygon);
    println!("Path: {path:?}");
    let mut list = DisplayList::<StaticPoint2d>::default();
    match &path {
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
                    points.push(start_line.pt_along(start_cut.polygon_pos()));
                    for p in cut_polygon.points_of_range(segment.copy_points()) {
                        points.push(*p);
                    }
                    points.push(end_line.pt_along(end_cut.polygon_pos()));
                    list.append_figure(Figure::from_points(points, "none", "red", 3));
                }
            }
        }
        PolygonPath::None => {}
    }

    let mut colors = ["blue", "green"].iter().cycle();
    let mut show = [true, true].iter().cycle();
    for polygon in &big_triangle.triangulate_cut_polygons(&small_triangle, &path)[0] {
        let stroke = colors.next().unwrap();
        let show = show.next().unwrap();
        if *show {
            list.append_figure(Figure::from_polygon(
                polygon.map(|p| p.coordinates_triangle()).clone(),
                "none",
                stroke,
                2,
            ));
        }
    }
    //list.append_figure(big_triangle.into());
    //list.append_figure(small_triangle.into());
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
#[test]
fn test_triangulate() {
    // A hollow square shape
    //  ________
    // |  ____  |
    // | |    | |
    // | |____| |
    // |________|
    let polygons = vec![
        vec![[0f32, 0f32], [0., 1.], [1., 1.], [1., 0.]],
        vec![[0.05, 0.05], [0.05, 0.95], [0.95, 0.95], [0.95, 0.05]],
    ];
    let mut triangulated_indices = Vec::<[usize; 2]>::new();
    polygons
        .triangulate(formats::IndexedListFormat::new(&mut triangulated_indices).into_fan_format())
        .expect("Triangulation failed");
    println!("indices: {triangulated_indices:?}");
    println!(
        "First triangle: {:?}, {:?}, {:?}",
        polygons.get_vertex(triangulated_indices[0]),
        polygons.get_vertex(triangulated_indices[1]),
        polygons.get_vertex(triangulated_indices[2])
    );
}
