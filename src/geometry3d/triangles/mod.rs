use std::hash::{Hash, Hasher};
use std::ptr;
use std::ptr::eq;

use stl_io::{IndexedMesh, IndexedTriangle, Vector, Vertex};

use crate::generic_triangle::{TriangleCornerPoint, TriangleSide};
use crate::geometry3d::plane::Plane3d;
use crate::geometry3d::point::Point3d;
use crate::geometry3d::triangle::Triangle3d;
use crate::geometry3d::triangles::indexed_point::IndexedPoint;

pub mod indexed_point;
pub mod topology;

#[derive(Clone, Debug, PartialEq, Hash)]
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
    pub fn triangles(&self) -> Vec<ReferencedTriangle<P>> {
        self.triangles
            .iter()
            .enumerate()
            .map(|(idx, entry)| self.create_triangle(idx, entry))
            .collect()
    }

    fn create_triangle(&self, idx: usize, entry: &IndexedTriangleEntry) -> ReferencedTriangle<P> {
        let p1 = IndexedPoint::new(self, entry.p1);
        let p2 = IndexedPoint::new(self, entry.p2);
        let p3 = IndexedPoint::new(self, entry.p3);
        ReferencedTriangle {
            list: self,
            idx,
            p1,
            p2,
            p3,
        }
    }
    pub fn get_triangle(&self, idx: usize) -> Option<ReferencedTriangle<P>> {
        self.triangles
            .get(idx)
            .map(|entry| self.create_triangle(idx, entry))
    }

    pub fn points(&self) -> &[P] {
        &self.points
    }

    pub fn transform_points<T, Pt>(self, transform: T) -> IndexedTriangleList<Pt>
    where
        T: FnMut(&P) -> Pt,
        Pt: Point3d,
    {
        IndexedTriangleList {
            points: self.points.iter().map(transform).collect(),
            triangles: self.triangles,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct IndexedTriangleEntry {
    p1: usize,
    p2: usize,
    p3: usize,
}

impl IndexedTriangleEntry {
    #[inline]
    fn get_point(&self, p: TriangleCornerPoint) -> usize {
        match p {
            TriangleCornerPoint::P1 => self.p1,
            TriangleCornerPoint::P2 => self.p2,
            TriangleCornerPoint::P3 => self.p3,
        }
    }
    #[inline]
    fn points(&self) -> [usize; 3] {
        [self.p1, self.p2, self.p3]
    }
    fn get_side(&self, side: TriangleSide) -> (usize, usize) {
        (
            self.get_point(side.start_corner()),
            self.get_point(side.end_corner()),
        )
    }
    fn sides(&self) -> [[usize; 2]; 3] {
        [[self.p1, self.p2], [self.p2, self.p3], [self.p3, self.p1]]
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ReferencedTriangle<'a, P: Point3d> {
    list: &'a IndexedTriangleList<P>,
    idx: usize,
    p1: IndexedPoint<'a, P>,
    p2: IndexedPoint<'a, P>,
    p3: IndexedPoint<'a, P>,
}

impl<'a, P: Point3d> Hash for ReferencedTriangle<'a, P> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        ptr::hash(self.list, state);
        state.write_usize(self.idx);
    }
}

impl<'a, P: Point3d> PartialEq for ReferencedTriangle<'a, P> {
    fn eq(&self, other: &Self) -> bool {
        eq(self.list, other.list) && self.idx == other.idx
    }
}

impl<'a, P: Point3d> Eq for ReferencedTriangle<'a, P> {}

impl<'a, P: Point3d> ReferencedTriangle<'a, P> {
    pub fn idx(&self) -> usize {
        self.idx
    }
}

impl<'a, P: Point3d> Triangle3d<IndexedPoint<'a, P>> for ReferencedTriangle<'a, P> {
    fn get_point(&self, p: TriangleCornerPoint) -> &IndexedPoint<'a, P> {
        match p {
            TriangleCornerPoint::P1 => &self.p1,
            TriangleCornerPoint::P2 => &self.p2,
            TriangleCornerPoint::P3 => &self.p3,
        }
    }

    fn points(&self) -> [&IndexedPoint<'a, P>; 3] {
        [&self.p1, &self.p2, &self.p3]
    }
}

impl From<IndexedMesh> for IndexedTriangleList<Vector<f32>> {
    fn from(value: IndexedMesh) -> Self {
        let mut builder = IndexedTriangleList::<Vector<f32>>::builder();
        for vertex in value.vertices {
            builder.append_point(vertex);
        }
        for face in value.faces {
            let verices = face.vertices;
            builder.append_indexed_triangle(verices[0], verices[1], verices[2]);
        }
        builder.build()
    }
}

impl<P: Point3d> From<IndexedTriangleList<P>> for IndexedMesh {
    fn from(value: IndexedTriangleList<P>) -> Self {
        let vertices = value
            .points
            .iter()
            .map(P::coordinates)
            .map(|c| [c.x.0 as f32, c.y.0 as f32, c.z.0 as f32])
            .map(Vertex::new)
            .collect();
        let faces = value
            .triangles
            .iter()
            .map(|tr| {
                let normal = if let (Some(p1), Some(p2), Some(p3)) = (
                    value.points.get(tr.p1),
                    value.points.get(tr.p2),
                    value.points.get(tr.p3),
                ) {
                    Plane3d::from_points(p1.coordinates(), p2.coordinates(), p3.coordinates())
                        .expect("Invalid plane in output")
                        .normal()
                } else {
                    panic!("Invalid triangle in index");
                };
                IndexedTriangle {
                    normal: Vector::new([normal.x.0 as f32, normal.y.0 as f32, normal.z.0 as f32]),
                    vertices: [tr.p1, tr.p2, tr.p3],
                }
            })
            .collect();
        IndexedMesh { vertices, faces }
    }
}
#[cfg(test)]
mod test {
    use std::fs::OpenOptions;

    use itertools::Itertools;
    use stl_io::IndexedMesh;

    use crate::geometry3d::triangles::topology::TriangleTopology;
    use crate::geometry3d::triangles::IndexedTriangleList;

    #[test]
    fn test_load_and_store() {
        let mut file = OpenOptions::new()
            .read(true)
            .open("test/Schublade - Front.stl")
            .unwrap();
        let stl = stl_io::read_stl(&mut file).unwrap();
        let triangle_list: IndexedTriangleList<_> = stl.clone().into();
        let topolgy = TriangleTopology::new(&triangle_list).expect("Error on topology");
        println!("Triangle count: {}", triangle_list.triangles().len());
        println!("Plane count: {}", topolgy.triangles_of_plane().len());
        let stats = topolgy
            .triangles_of_plane()
            .values()
            .map(|tr| tr.len())
            .counts();
        for (triangle_count, plane_count) in stats {
            println!("{plane_count} Planes with {triangle_count} Triangles");
        }
        for (plane, triangles) in topolgy.triangles_of_plane() {
            if triangles.len() > 3 {
                println!("Plane: {:?}: {}", plane, triangles.len())
            }
        }

        let new_stl_data: IndexedMesh = triangle_list.into();
        // compare vertices
        assert_eq!(stl.vertices, new_stl_data.vertices);
        let triangles = [stl, new_stl_data]
            .iter()
            .map(|mesh| mesh.faces.iter().map(|f| f.vertices).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        // compare triangles (without normals)
        assert_eq!(triangles.get(0), triangles.get(1));
    }
}
