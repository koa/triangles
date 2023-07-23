use crate::generic_triangle::TriangleCornerPoint;
use crate::geometry3d::point::Point3d;
use crate::geometry3d::triangle::Triangle3d;

#[derive(Clone, Debug, PartialEq)]
pub struct IndexedTriangleList<P: Point3d> {
    points: Box<[P]>,
    triangles: Box<[IndexedTriangleEntry]>,
}

pub struct TriangleListBuilder<P: Point3d> {
    points: Vec<P>,
    triangles: Vec<IndexedTriangleEntry>,
}

impl<P: Point3d> TriangleListBuilder<P> {
    fn append_point(&mut self, point: P) -> usize {
        self.points.push(point);
        self.points.len() - 1
    }
    fn append_indexed_triangle(&mut self, p1: usize, p2: usize, p3: usize) -> usize {
        assert!(p1 < self.points.len());
        assert!(p2 < self.points.len());
        assert!(p3 < self.points.len());
        self.triangles.push(IndexedTriangleEntry { p1, p2, p3 });
        self.triangles.len() - 1
    }
    fn build(self) -> IndexedTriangleList<P> {
        let points = self.points.into_boxed_slice();
        let triangles = self.triangles.into_boxed_slice();
        IndexedTriangleList { points, triangles }
    }
}

impl<P: Point3d> IndexedTriangleList<P> {
    fn builder<Pn: Point3d>() -> TriangleListBuilder<Pn> {
        TriangleListBuilder {
            points: Vec::<Pn>::new(),
            triangles: vec![],
        }
    }
    fn triangles(&self) -> Vec<ReferencedTriangle<P>> {
        self.triangles
            .iter()
            .enumerate()
            .filter_map(|(idx, entry)| self.create_triangle(idx, entry))
            .collect()
    }

    fn create_triangle(
        &self,
        idx: usize,
        entry: &IndexedTriangleEntry,
    ) -> Option<ReferencedTriangle<P>> {
        if let (Some(p1), Some(p2), Some(p3)) = (
            self.points.get(entry.p1),
            self.points.get(entry.p2),
            self.points.get(entry.p3),
        ) {
            Some(ReferencedTriangle {
                list: self,
                idx,
                p1,
                p2,
                p3,
            })
        } else {
            None
        }
    }
    fn get_triangle(&self, idx: usize) -> Option<ReferencedTriangle<P>> {
        self.triangles
            .get(idx)
            .and_then(|entry| self.create_triangle(idx, entry))
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct IndexedTriangleEntry {
    p1: usize,
    p2: usize,
    p3: usize,
}

impl IndexedTriangleEntry {
    fn get_point(&self, p: TriangleCornerPoint) -> usize {
        match p {
            TriangleCornerPoint::P1 => self.p1,
            TriangleCornerPoint::P2 => self.p2,
            TriangleCornerPoint::P3 => self.p3,
        }
    }
    fn points(&self) -> [usize; 3] {
        [self.p1, self.p2, self.p3]
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReferencedTriangle<'a, P: Point3d> {
    list: &'a IndexedTriangleList<P>,
    idx: usize,
    p1: &'a P,
    p2: &'a P,
    p3: &'a P,
}

impl<'a, P: Point3d> ReferencedTriangle<'a, P> {
    pub fn idx(&self) -> usize {
        self.idx
    }
}

impl<'a, P: Point3d> Triangle3d<P> for ReferencedTriangle<'a, P> {
    fn get_point(&self, p: TriangleCornerPoint) -> &P {
        match p {
            TriangleCornerPoint::P1 => self.p1,
            TriangleCornerPoint::P2 => self.p2,
            TriangleCornerPoint::P3 => self.p3,
        }
    }

    fn points(&self) -> [&P; 3] {
        [self.p1, self.p2, self.p3]
    }
}
