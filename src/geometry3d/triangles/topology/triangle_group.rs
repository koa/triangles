use ordered_float::OrderedFloat;
use std::collections::{HashMap, HashSet};
use stl_io::Vector;

use crate::geometry3d::line::static_line::PointLine3d;
use crate::geometry3d::triangles::indexed_point::IndexedPoint;
use crate::prelude::{Line3d, Plane3d, Point3d, ReferencedTriangle, Triangle3d};

#[derive(Debug, Clone, Default)]
struct TriangleGroup<'a, P: Point3d> {
    triangles: Vec<ReferencedTriangle<'a, P>>,
    edges: Vec<Vec<PointLine3d<IndexedPoint<'a, P>>>>,
}

impl<'a, P: Point3d> TriangleGroup<'a, P> {
    pub fn new(triangles: Vec<ReferencedTriangle<'a, P>>) -> Self {
        let mut outer_edges = HashSet::new();
        for triangle in &triangles {
            let sides = triangle.sides();
            for side in sides.iter().map(|s| s.deref()) {
                if !outer_edges.remove(&side.clone().reverse()) {
                    outer_edges.insert(side);
                }
            }
        }
        let mut edges = Vec::with_capacity(1);
        while let Some(start_line) = outer_edges.iter().next().cloned() {
            let mut segments = Vec::with_capacity(outer_edges.len());
            outer_edges.remove(&start_line);
            let mut next_pt = start_line.p2();
            let start_pt = start_line.p1();
            segments.push(start_line);
            while next_pt != start_pt {
                let next_line = outer_edges
                    .iter()
                    .find(|line| line.p1() == next_pt)
                    .expect("Invalid Topology")
                    .clone();
                next_pt = next_line.p2().clone();
                outer_edges.remove(&next_line);
                segments.push(next_line);
            }
            segments.shrink_to_fit();
            edges.push(segments);
        }

        Self { triangles, edges }
    }

    pub fn triangles(&self) -> &Vec<ReferencedTriangle<'a, P>> {
        &self.triangles
    }
    pub fn edges(&self) -> &Vec<Vec<PointLine3d<IndexedPoint<'a, P>>>> {
        &self.edges
    }
}

#[cfg(test)]
mod test {
    use crate::geometry3d::triangles::topology::triangle_group::TriangleGroup;
    use crate::prelude::{Line3d, TriangleTopology};
    use crate::test::load_schublade_as_triangles;

    #[test]
    fn test_create_triangle_groups() {
        let triangles = load_schublade_as_triangles();
        let topology = TriangleTopology::new(&triangles).expect("Topology error");
        for (plane, triangles) in topology.triangles_of_plane {
            let group = TriangleGroup::new(triangles);
            println!("Plane: {},{}", plane.normal(), plane.distance());
            for edge in group.edges {
                let points: Vec<_> = edge.iter().map(|l| l.p1()).map(|p| p.idx()).collect();
                println!("  Edges: {:?}", points);
            }
        }
    }
}
