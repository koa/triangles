mod generic_triangle;
mod geometry2d;
mod geometry3d;
mod io;
mod primitives;
pub mod test;
pub mod prelude {
    pub use crate::{
        geometry2d::{
            line::Line2d,
            point::{
                bounding_box::{BoundingBox2d, BoundingBox2dValues},
                Point2d, StaticPoint2d,
            },
            polygon::{cut::PolygonPath, AnyPolygon, Polygon2d},
            triangle::{static_triangle::StaticTriangle2d, Triangle2d},
        },
        geometry3d::{
            line::{static_line::StaticLine3d, Line3d},
            plane::{projection::PlaneProjection, Plane3d},
            point::{point_3d, Point3d},
            triangle::Triangle3d,
            triangles::{topology::TriangleTopology, IndexedTriangleList, ReferencedTriangle},
            Vector3d,
        },
        primitives::{Float, Number},
    };
}
