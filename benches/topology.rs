use criterion::{criterion_group, criterion_main, Criterion};

use triangles::prelude::TriangleTopology;
use triangles::test::load_schublade_as_triangles;

pub fn append_benches(c: &mut Criterion) {
    let triangles = load_schublade_as_triangles();
    c.bench_function("Load Topology from stl file", |b| {
        b.iter(load_schublade_as_triangles)
    });

    c.bench_function("Create Topology from triangle list", |b| {
        b.iter(|| TriangleTopology::new(&triangles))
    });
}
