use std::io::Cursor;

use stl_io::{IndexedMesh, Vector};

use crate::prelude::IndexedTriangleList;

pub fn load_schublade() -> IndexedMesh {
    let bytes = include_bytes!("Schublade - Front.stl");
    let mut cursor = Cursor::new(bytes);
    stl_io::read_stl(&mut cursor).expect("Cannot load stl file")
}

pub fn load_schublade_as_triangles() -> IndexedTriangleList<Vector<f32>> {
    load_schublade().into()
}
