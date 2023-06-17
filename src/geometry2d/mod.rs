pub use point::Point2d;
pub use polygon::Polygon2d;
pub use triangle::{StaticTriangle2d, Triangle2d};

mod line;
mod point;
mod polygon;
#[cfg(test)]
mod test;
mod triangle;
mod vector;
