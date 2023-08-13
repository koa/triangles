use std::fmt::Debug;

use ordered_float::OrderedFloat;
use stl_io::Vertex;
pub mod bounding_box;

use crate::geometry3d::Vector3d;
use crate::primitives::Number;

pub trait Point3d: Sized + Debug + PartialEq + Clone {
    fn coordinates(&self) -> Vector3d;
    fn dist_square<P: Point3d>(&self, other: &P) -> Number {
        self.coordinates().distance_squared(other.coordinates())
    }
}

pub fn point_3d<F1, F2, F3>(x: F1, y: F2, z: F3) -> Vector3d
where
    OrderedFloat<f64>: From<F1>,
    OrderedFloat<f64>: From<F2>,
    OrderedFloat<f64>: From<F3>,
{
    Vector3d::new(x.into(), y.into(), z.into())
}

impl Point3d for Vector3d {
    fn coordinates(&self) -> Vector3d {
        *self
    }
}

impl Point3d for Vertex {
    fn coordinates(&self) -> Vector3d {
        point_3d(self[0] as f64, self[1] as f64, self[2] as f64)
    }
}
