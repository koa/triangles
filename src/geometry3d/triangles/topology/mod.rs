use std::collections::{HashMap, HashSet};

use num_traits::Zero;
use ordered_float::OrderedFloat;
use thiserror::Error;

use crate::{
    geometry3d::{
        line::{static_line::PointLine3d, Line3d},
        plane::{InvalidPlane, Plane3d},
        point::Point3d,
        triangle::Triangle3d,
        triangles::{indexed_point::IndexedPoint, IndexedTriangleList, ReferencedTriangle},
        Vector3d,
    },
    prelude::{PlaneProjection, Point2d, Triangle2d},
    primitives::Number,
};

pub mod triangle_group;

pub struct TriangleTopology<'a, P: Point3d> {
    //triangles: &'a IndexedTriangleList<P>,
    edge_neighbors: HashMap<PointLine3d<IndexedPoint<'a, P>>, [ReferencedTriangle<'a, P>; 2]>,
    triangles_of_plane: HashMap<Plane3d, Vec<ReferencedTriangle<'a, P>>>,
    plane_groups: HashMap<Plane3d, Vec<Plane3d>>,
}

impl<'a, P: Point3d> TriangleTopology<'a, P> {
    pub fn new(triangles: &'a IndexedTriangleList<P>) -> Result<Self, TopologyError<'a, P>> {
        let mut collecting_neighbors = HashMap::<_, LineNeighbors<'a, P>>::new();
        let mut triangles_of_plane = HashMap::<_, Vec<_>>::new();
        for triangle in triangles.triangles().iter() {
            for side in triangle.sides() {
                let p1 = side.p1();
                let p2 = side.p2();
                let (entry, key) = if p1.idx() < p2.idx() {
                    let key = PointLine3d::new(p1, p2);
                    (
                        collecting_neighbors
                            .get(&key)
                            .cloned()
                            .unwrap_or_default()
                            .forward(triangle.clone())?,
                        key,
                    )
                } else {
                    let key = PointLine3d::new(p2, p1);
                    (
                        collecting_neighbors
                            .get(&key)
                            .cloned()
                            .unwrap_or_default()
                            .backward(triangle.clone())?,
                        key,
                    )
                };
                collecting_neighbors.insert(key, entry);
            }
            let plane = triangle.calculate_plane()?;
            triangles_of_plane
                .entry(plane)
                .or_default()
                .push(triangle.clone());
        }
        let mut edge_neighbors = HashMap::with_capacity(collecting_neighbors.capacity());
        for (edge, result) in collecting_neighbors {
            edge_neighbors.insert(edge, result.triangles_tuple()?);
        }
        let plane_groups = group_planes(&triangles_of_plane);

        Ok(Self {
            //triangles,
            edge_neighbors,
            triangles_of_plane,
            plane_groups,
        })
    }

    pub fn edge_neighbors(
        &self,
    ) -> &HashMap<PointLine3d<IndexedPoint<'a, P>>, [ReferencedTriangle<'a, P>; 2]> {
        &self.edge_neighbors
    }
    pub fn triangles_of_plane(&self) -> &HashMap<Plane3d, Vec<ReferencedTriangle<'a, P>>> {
        &self.triangles_of_plane
    }

    pub fn find_first_intersection<L: Line3d<Pt>, Pt: Point3d>(
        &'a self,
        sight_line: &L,
    ) -> Option<(Vector3d, &'a Plane3d, &'a ReferencedTriangle<'a, P>)> {
        let camera_pos = sight_line.p1();
        let mut found_planes = Vec::with_capacity(self.triangles_of_plane.len());
        for (plane, triangles) in self.triangles_of_plane.iter() {
            let p = plane.intersect_line_at(sight_line);
            if p > Number::zero() && plane.is_in_front(&camera_pos) {
                found_planes.push((p, plane, triangles));
            }
        }
        found_planes.sort_by_key(|(p, _, _)| *p);
        for (p, plane, triangles) in found_planes {
            let project = PlaneProjection::new(plane);
            let point_on_plane = sight_line.point_at(p);
            let click_point_2d = project.project_point(&point_on_plane).coordinates();
            for triangle in triangles {
                let triangle_2d = project.project_triangle(triangle);

                if triangle_2d.contains_pt(&click_point_2d) {
                    return Some((point_on_plane, plane, triangle));
                }
            }
        }
        None
    }

    pub fn plane_groups(&self) -> &HashMap<Plane3d, Vec<Plane3d>> {
        &self.plane_groups
    }
}

#[derive(Default, Copy, Clone)]
enum LineNeighbors<'a, P: Point3d> {
    #[default]
    None,
    OnlyForward(ReferencedTriangle<'a, P>),
    OnlyBackward(ReferencedTriangle<'a, P>),
    Both {
        forward: ReferencedTriangle<'a, P>,
        backward: ReferencedTriangle<'a, P>,
    },
}

#[derive(Error, Debug, Clone)]
pub enum TopologyError<'a, P: Point3d> {
    #[error("Two triangles on the same side of a edge: {},{}",.0[0].idx(),.0[1].idx())]
    DuplicateNeighborEntry([ReferencedTriangle<'a, P>; 2]),
    #[error("Triangle {} has no neighbor at one edge",.0.idx())]
    MissingNeighborError(ReferencedTriangle<'a, P>),
    #[error("Invalid plane found")]
    InvalidPlane(#[from] InvalidPlane),
}

impl<'a, P: Point3d> LineNeighbors<'a, P> {
    fn forward(self, idx: ReferencedTriangle<'a, P>) -> Result<Self, TopologyError<'a, P>> {
        match self {
            LineNeighbors::None => Ok(LineNeighbors::OnlyForward(idx)),
            LineNeighbors::OnlyForward(other) => {
                Err(TopologyError::DuplicateNeighborEntry([other, idx]))
            }
            LineNeighbors::OnlyBackward(backward) => Ok(LineNeighbors::Both {
                forward: idx,
                backward,
            }),
            LineNeighbors::Both { forward, .. } => {
                Err(TopologyError::DuplicateNeighborEntry([forward, idx]))
            }
        }
    }
    fn backward(self, idx: ReferencedTriangle<'a, P>) -> Result<Self, TopologyError<'a, P>> {
        match self {
            LineNeighbors::None => Ok(LineNeighbors::OnlyBackward(idx)),
            LineNeighbors::OnlyBackward(backward) => {
                Err(TopologyError::DuplicateNeighborEntry([backward, idx]))
            }
            LineNeighbors::OnlyForward(forward) => Ok(LineNeighbors::Both {
                forward,
                backward: idx,
            }),
            LineNeighbors::Both { backward, .. } => {
                Err(TopologyError::DuplicateNeighborEntry([backward, idx]))
            }
        }
    }
    fn triangles_tuple(self) -> Result<[ReferencedTriangle<'a, P>; 2], TopologyError<'a, P>> {
        match self {
            LineNeighbors::None => {
                panic!("Should never happen")
            }
            LineNeighbors::OnlyForward(idx) => Err(TopologyError::MissingNeighborError(idx)),
            LineNeighbors::OnlyBackward(idx) => Err(TopologyError::MissingNeighborError(idx)),
            LineNeighbors::Both { forward, backward } => Ok([forward, backward]),
        }
    }
}

fn group_planes<P: Point3d>(
    triangles: &HashMap<Plane3d, Vec<ReferencedTriangle<P>>>,
) -> HashMap<Plane3d, Vec<Plane3d>> {
    let mut planes = Vec::with_capacity(triangles.len());
    for first_plane in triangles.keys() {
        let mut density = 0.0;
        for second_plane in triangles.keys() {
            if first_plane == second_plane {
                continue;
            }
            let dist = first_plane.dist_square(second_plane);
            /*if dist > (100.0 * f32::EPSILON) as f64 {
                continue;
            }*/
            density += 1.0 / dist.0;
        }
        //println!("Plane: {first_plane:?}: {density}");
        planes.push((density, first_plane));
    }
    planes.sort_by_key(|(d, _)| OrderedFloat::from(-*d));

    let mut remaining_planes: HashSet<_> = triangles.keys().collect();
    let mut grouped_planes = HashMap::new();
    const PLANE_GROUP_THRESHOLD: f64 = 500.0 * f64::EPSILON;

    for (_, plane) in planes {
        if !remaining_planes.remove(plane) {
            continue;
        }
        let mut neighbors = vec![*plane];
        let mut max_dist: f64 = 0.0;
        for candidate in remaining_planes.iter() {
            let x = plane.dist_square(candidate).0;
            if x < PLANE_GROUP_THRESHOLD {
                //println!("  {}: {candidate:?}: {x}", PLANE_GROUP_THRESHOLD / x);
                max_dist = max_dist.max(x);
                neighbors.push(**candidate);
            }
        }
        for p in neighbors.iter() {
            remaining_planes.remove(p);
        }

        grouped_planes.insert(*plane, neighbors);
    }
    grouped_planes
}

#[cfg(test)]
mod test;
