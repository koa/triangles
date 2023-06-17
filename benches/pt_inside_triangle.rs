use criterion::{criterion_group, criterion_main, Criterion};
use ordered_float::OrderedFloat;
use rand::prelude::ThreadRng;
use rand::Rng;

use triangles::geometry2d::{Point2d, StaticTriangle2d, Triangle2d};

fn create_triangles(count: usize, random: &mut ThreadRng) -> Vec<StaticTriangle2d> {
    let mut ret = Vec::with_capacity(count);
    while ret.len() < count {
        let candidate =
            StaticTriangle2d::new(random_pt(random), random_pt(random), random_pt(random));
        if candidate.area() > OrderedFloat::from(0.0) {
            ret.push(candidate);
        }
    }
    ret
}

fn random_pt(random: &mut ThreadRng) -> Point2d {
    (random.gen_range(0.0..1.0), random.gen_range(0.0..1.0)).into()
}

fn triangle_contains(triangles: &[StaticTriangle2d], points: &[Point2d]) {
    for triangle in triangles {
        for point in points {
            triangle.contains_pt(point);
        }
    }
}
fn polygon_contains(triangles: &[StaticTriangle2d], points: &[Point2d]) {
    for triangle in triangles {
        for point in points {
            triangle.point_position(point);
        }
    }
}
use triangles::geometry2d::Polygon2d;
fn criterion_benchmark(c: &mut Criterion) {
    let mut random = rand::thread_rng();
    let triangles = create_triangles(500, &mut random);
    let points: Vec<_> = (0..100).map(|_| random_pt(&mut random)).collect();
    c.bench_function("triangle_contains_pt", |b| {
        b.iter(|| triangle_contains(&triangles, &points))
    });
    c.bench_function("polygon_contains_pt", |b| {
        b.iter(|| polygon_contains(&triangles, &points))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
