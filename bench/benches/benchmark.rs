use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};
use bench::benchtest;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bench cow", |b| b.iter(|| benchtest(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
