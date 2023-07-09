mod geometry2d;
mod primitives;
pub mod prelude {
    pub use crate::{
        geometry2d::{
            line::Line2d,
            point::{BoundingBox, BoundingBoxValues, Point2d},
            polygon::{cut::PolygonPath, AnyPolygon, Polygon2d},
            triangle::{static_triangle::StaticTriangle2d, Triangle2d},
        },
        primitives::{Float, Number},
    };
}
