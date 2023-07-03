mod geometry2d;
mod primitives;
pub mod prelude {
    pub use crate::{
        geometry2d::{
            point::{BoundingBox, BoundingBoxValues, Point2d},
            polygon::{AnyPolygon, Polygon2d},
            triangle::static_triangle::StaticTriangle2d,
        },
        primitives::Number,
    };
}
