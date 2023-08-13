use std::fmt::Debug;
use std::marker::PhantomData;

use triangulate::Vertex;

use crate::geometry2d::triangle::{TriangleCornerPoint, TrianglePointIterator, TriangleSide};
use crate::prelude::{Number, Point2d, Polygon2d, StaticPoint2d, StaticTriangle2d, Triangle2d};

pub fn found_original_triangle<
    't,
    T: Triangle2d<PointTriangle> + 't,
    PointTriangle: Point2d,
    P: Polygon2d<PointPolygon> + 't,
    PointPolygon: Point2d,
>(
    triangle: &'t T,
) -> FoundTriangle<'t, T, PointTriangle, P, PointPolygon> {
    FoundTriangle::Original {
        triangle,
        p1: found_triangle_corner(TriangleCornerPoint::P1, triangle),
        p2: found_triangle_corner(TriangleCornerPoint::P2, triangle),
        p3: found_triangle_corner(TriangleCornerPoint::P3, triangle),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FoundTriangle<
    't,
    T: Triangle2d<PointTriangle> + 't,
    PointTriangle: Point2d,
    P: Polygon2d<PointPolygon> + 't,
    PointPolygon: Point2d,
> {
    Original {
        triangle: &'t T,
        p1: FoundPoint<'t, PointTriangle, PointPolygon, T, P>,
        p2: FoundPoint<'t, PointTriangle, PointPolygon, T, P>,
        p3: FoundPoint<'t, PointTriangle, PointPolygon, T, P>,
    },
    FoundTriangle(StaticTriangle2d<FoundPoint<'t, PointTriangle, PointPolygon, T, P>>),
    _Phantom(
        PhantomData<PointTriangle>,
        PhantomData<P>,
        PhantomData<PointPolygon>,
    ),
}

impl<
        't,
        T: Triangle2d<PointTriangle> + 't,
        PointTriangle: Point2d,
        P: Polygon2d<PointPolygon> + 't,
        PointPolygon: Point2d,
    > FoundTriangle<'t, T, PointTriangle, P, PointPolygon>
{
    pub fn coordinates_triangle(&self) -> StaticTriangle2d<StaticPoint2d> {
        StaticTriangle2d::new(
            self.p1().coordinates(),
            self.p2().coordinates(),
            self.p3().coordinates(),
        )
    }
}

impl<
        't,
        T: Triangle2d<PointTriangle> + 't,
        PointTriangle: Point2d + 't,
        P: Polygon2d<PointPolygon> + 't,
        PointPolygon: Point2d + 't,
    > Polygon2d<FoundPoint<'t, PointTriangle, PointPolygon, T, P>>
    for FoundTriangle<'t, T, PointTriangle, P, PointPolygon>
{
    type PointIter<'a> = TrianglePointIterator<
        'a,
        FoundTriangle<'t, T, PointTriangle, P, PointPolygon>,
        FoundPoint<'t, PointTriangle, PointPolygon, T, P>,
    > where 't: 'a;

    fn points(&self) -> Self::PointIter<'_> {
        TrianglePointIterator {
            phantom: Default::default(),
            triangle: self,
            state: Default::default(),
        }
    }

    fn point_count(&self) -> usize {
        3
    }

    fn get_point(&self, idx: usize) -> Option<&FoundPoint<'t, PointTriangle, PointPolygon, T, P>> {
        <usize as TryInto<TriangleCornerPoint>>::try_into(idx)
            .ok()
            .map(|p| self.point(p))
    }
}

impl<
        't,
        T: Triangle2d<PointTriangle> + 't,
        PointTriangle: Point2d + 't,
        P: Polygon2d<PointPolygon> + 't,
        PointPolygon: Point2d + 't,
    > Triangle2d<FoundPoint<'t, PointTriangle, PointPolygon, T, P>>
    for FoundTriangle<'t, T, PointTriangle, P, PointPolygon>
{
    fn p1(&self) -> &FoundPoint<'t, PointTriangle, PointPolygon, T, P> {
        self.point(TriangleCornerPoint::P1)
    }

    fn p2(&self) -> &FoundPoint<'t, PointTriangle, PointPolygon, T, P> {
        self.point(TriangleCornerPoint::P2)
    }

    fn p3(&self) -> &FoundPoint<'t, PointTriangle, PointPolygon, T, P> {
        self.point(TriangleCornerPoint::P3)
    }
    fn point(&self, p: TriangleCornerPoint) -> &FoundPoint<'t, PointTriangle, PointPolygon, T, P> {
        match self {
            FoundTriangle::Original {
                triangle: _,
                p1,
                p2,
                p3,
            } => match p {
                TriangleCornerPoint::P1 => p1,
                TriangleCornerPoint::P2 => p2,
                TriangleCornerPoint::P3 => p3,
            },
            FoundTriangle::FoundTriangle(t) => t.point(p),
            FoundTriangle::_Phantom(_, _, _) => panic!("Never use"),
        }
    }

    /*
    fn reverse(&self) -> Self {
        match self {
            FoundTriangle::Original(t) =>  FoundTriangle::Original{t.},
            FoundTriangle::FoundTriangle(t) => FoundTriangle::FoundTriangle(t.reverse()),
            FoundTriangle::_Phantom(_, _, _) => panic!("Never use"),
        }
    }*/
}

pub fn found_triangle_corner<
    PointTriangle: Point2d,
    PointPolygon: Point2d,
    T: Triangle2d<PointTriangle>,
    P: Polygon2d<PointPolygon>,
>(
    corner: TriangleCornerPoint,
    triangle: &T,
) -> FoundPoint<PointTriangle, PointPolygon, T, P> {
    FoundPoint::TriangleCorner {
        corner,
        triangle,
        phantom: Default::default(),
    }
}
pub fn found_cross_point<
    't,
    PointTriangle: Point2d,
    PointPolygon: Point2d,
    T: Triangle2d<PointTriangle>,
    P: Polygon2d<PointPolygon>,
>(
    triangle_side: TriangleSide,
    triangle: &'t T,
    polygon_idx: usize,
    polygon: &'t P,
    coordinates: StaticPoint2d,
) -> FoundPoint<'t, PointTriangle, PointPolygon, T, P> {
    FoundPoint::CrossPoint {
        triangle_side,
        triangle,
        polygon_idx,
        polygon,
        coordinates,
    }
}

pub fn found_polygon_point<
    PointTriangle: Point2d,
    PointPolygon: Point2d,
    T: Triangle2d<PointTriangle>,
    P: Polygon2d<PointPolygon>,
>(
    idx: usize,
    polygon: &P,
) -> FoundPoint<PointTriangle, PointPolygon, T, P> {
    FoundPoint::PolygonPoint {
        idx,
        polygon,
        phantom: Default::default(),
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FoundPoint<
    't,
    PointTriangle: Point2d,
    PointPolygon: Point2d,
    T: Triangle2d<PointTriangle>,
    P: Polygon2d<PointPolygon>,
> {
    TriangleCorner {
        corner: TriangleCornerPoint,
        triangle: &'t T,
        phantom: PhantomData<PointTriangle>,
    },
    PolygonPoint {
        idx: usize,
        polygon: &'t P,
        phantom: PhantomData<PointPolygon>,
    },
    CrossPoint {
        triangle_side: TriangleSide,
        triangle: &'t T,
        polygon_idx: usize,
        polygon: &'t P,
        coordinates: StaticPoint2d,
    },
}

impl<'t, Pt: Point2d, Pt2: Point2d, T: Triangle2d<Pt>, P: Polygon2d<Pt2>> Point2d
    for FoundPoint<'t, Pt, Pt2, T, P>
{
    fn coordinates(&self) -> StaticPoint2d {
        match self {
            FoundPoint::TriangleCorner {
                corner,
                triangle,
                phantom: _phantom,
            } => triangle.point(*corner).coordinates(),
            FoundPoint::PolygonPoint {
                idx,
                polygon,
                phantom: _phantom,
            } => polygon
                .get_point(*idx)
                .expect("Index out of range")
                .coordinates(),
            FoundPoint::CrossPoint { coordinates, .. } => *coordinates,
        }
    }
}

impl<Pt2: Point2d, Pt: Point2d, T: Triangle2d<Pt>, P: Polygon2d<Pt2>> Vertex
    for FoundPoint<'_, Pt, Pt2, T, P>
{
    type Coordinate = Number;

    fn x(&self) -> Self::Coordinate {
        self.coordinates().x
    }

    fn y(&self) -> Self::Coordinate {
        self.coordinates().y
    }
}
