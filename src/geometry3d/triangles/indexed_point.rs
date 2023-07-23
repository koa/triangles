use std::ptr::eq;

use crate::geometry3d::point::Point3d;
use crate::geometry3d::triangles::IndexedTriangleList;
use crate::geometry3d::Vector3d;

#[derive(Copy, Clone, Debug, Hash)]
pub struct IndexedPoint<'a, P: Point3d> {
    list: &'a IndexedTriangleList<P>,
    idx: usize,
    p: &'a P,
}

impl<'a, P: Point3d> IndexedPoint<'a, P> {
    pub fn new(list: &'a IndexedTriangleList<P>, idx: usize) -> Self {
        let p = list.points.get(idx).expect("Missing referenced point");
        Self { list, idx, p }
    }
}

impl<'a, P: Point3d> IndexedPoint<'a, P> {
    pub fn idx(&self) -> usize {
        self.idx
    }
}

impl<'a, P: Point3d> Point3d for IndexedPoint<'a, P> {
    fn coordinates(&self) -> Vector3d {
        self.p.coordinates()
    }
}

impl<'a, P: Point3d> PartialEq for IndexedPoint<'a, P> {
    fn eq(&self, other: &Self) -> bool {
        eq(self.list, other.list) && self.idx == other.idx
    }
}

impl<'a, P: Point3d> Eq for IndexedPoint<'a, P> {}
