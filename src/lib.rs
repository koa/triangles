mod generic_triangle;
mod geometry2d;
mod geometry3d;
mod io;
mod primitives;
pub mod prelude {
    pub use crate::{
        geometry2d::{
            line::Line2d,
            point::{
                bounding_box::{BoundingBox, BoundingBoxValues},
                Point2d, StaticPoint2d,
            },
            polygon::{cut::PolygonPath, AnyPolygon, Polygon2d},
            triangle::{static_triangle::StaticTriangle2d, Triangle2d},
        },
        geometry3d::{
            point::{point_3d, Point3d},
            triangle::Triangle3d,
            triangles::{topology::TriangleTopology, IndexedTriangleList},
            Vector3d,
        },
        primitives::{Float, Number},
    };
}
