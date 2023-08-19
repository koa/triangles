use criterion::{criterion_group, criterion_main, Criterion};

mod figures2d;
mod topology;

fn criterion_benchmark(c: &mut Criterion) {
    figures2d::append_benches(c);
    topology::append_benches(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
