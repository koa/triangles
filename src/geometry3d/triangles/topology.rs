use std::collections::HashMap;

use thiserror::Error;

use crate::geometry3d::point::Point3d;
use crate::geometry3d::triangles::IndexedTriangleList;

pub struct TriangleTopolgy<'a, P: Point3d> {
    triangles: &'a IndexedTriangleList<P>,
    edge_neighbors: HashMap<(usize, usize), (usize, usize)>,
}

impl<'a, P: Point3d> TriangleTopolgy<'a, P> {
    pub fn new(triangles: &'a IndexedTriangleList<P>) -> Result<Self, TopologyError> {
        let mut collecting_neighbors = HashMap::<_, LineNeighbors>::new();
        for (idx, idx_triangle) in triangles.triangles.iter().enumerate() {
            for [p1, p2] in idx_triangle.sides() {
                let (key, entry) = if p1 < p2 {
                    let key = (p1, p2);
                    (
                        key,
                        collecting_neighbors
                            .get(&key)
                            .cloned()
                            .unwrap_or_default()
                            .forward(idx)?,
                    )
                } else {
                    let key = (p2, p1);
                    (
                        key,
                        collecting_neighbors
                            .get(&key)
                            .cloned()
                            .unwrap_or_default()
                            .backward(idx)?,
                    )
                };
                collecting_neighbors.insert(key, entry);
            }
        }
        let mut edge_neighbors = HashMap::with_capacity(collecting_neighbors.capacity());
        for (edge, result) in collecting_neighbors {
            edge_neighbors.insert(edge, result.triangles_tuple()?);
        }
        Ok(Self {
            triangles,
            edge_neighbors,
        })
    }
}

pub struct NeighborTuple {
    forward: usize,
    backward: usize,
}

#[derive(Default, Copy, Clone)]
enum LineNeighbors {
    #[default]
    None,
    OnlyForward(usize),
    OnlyBackward(usize),
    Both {
        forward: usize,
        backward: usize,
    },
}

#[derive(Error, Debug)]
pub enum TopologyError {
    #[error("Two triangles on the same side of a edge: {},{}",.0[0],.0[1])]
    DuplicateNeighborEntry([usize; 2]),
    #[error("Triangle {idx} has no neighbor one edge")]
    MissingNeighborError { idx: usize },
}

impl LineNeighbors {
    fn forward(self, idx: usize) -> Result<Self, TopologyError> {
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
    fn backward(self, idx: usize) -> Result<Self, TopologyError> {
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
    fn lonely_triangle(&self) -> Option<usize> {
        match self {
            LineNeighbors::None => None,
            LineNeighbors::OnlyForward(idx) => Some(*idx),
            LineNeighbors::OnlyBackward(idx) => Some(*idx),
            LineNeighbors::Both { .. } => None,
        }
    }
    fn triangles_tuple(&self) -> Result<(usize, usize), TopologyError> {
        match self {
            LineNeighbors::None => {
                panic!("Should never happen")
            }
            LineNeighbors::OnlyForward(idx) => {
                Err(TopologyError::MissingNeighborError { idx: *idx })
            }
            LineNeighbors::OnlyBackward(idx) => {
                Err(TopologyError::MissingNeighborError { idx: *idx })
            }
            LineNeighbors::Both { forward, backward } => Ok((*forward, *backward)),
        }
    }
}
