use std::collections::HashMap;

use thiserror::Error;

use crate::geometry3d::line::static_line::PointLine3d;
use crate::geometry3d::line::Line3d;
use crate::geometry3d::plane::{InvalidPlane, Plane3d};
use crate::geometry3d::point::Point3d;
use crate::geometry3d::triangle::Triangle3d;
use crate::geometry3d::triangles::indexed_point::IndexedPoint;
use crate::geometry3d::triangles::{IndexedTriangleList, ReferencedTriangle};

pub struct TriangleTopolgy<'a, P: Point3d> {
    triangles: &'a IndexedTriangleList<P>,
    edge_neighbors: HashMap<PointLine3d<IndexedPoint<'a, P>>, [ReferencedTriangle<'a, P>; 2]>,
    triangles_of_plane: HashMap<Plane3d, Vec<ReferencedTriangle<'a, P>>>,
}

impl<'a, P: Point3d> TriangleTopolgy<'a, P> {
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
        Ok(Self {
            triangles,
            edge_neighbors,
            triangles_of_plane,
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
}

pub struct NeighborTuple {
    forward: usize,
    backward: usize,
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
    fn triangles_tuple(&self) -> Result<[ReferencedTriangle<'a, P>; 2], TopologyError<'a, P>> {
        match self {
            LineNeighbors::None => {
                panic!("Should never happen")
            }
            LineNeighbors::OnlyForward(idx) => {
                Err(TopologyError::MissingNeighborError(idx.clone()))
            }
            LineNeighbors::OnlyBackward(idx) => {
                Err(TopologyError::MissingNeighborError(idx.clone()))
            }
            LineNeighbors::Both { forward, backward } => Ok([forward.clone(), backward.clone()]),
        }
    }
}
