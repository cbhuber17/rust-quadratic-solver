use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_quadratic_solver::quadratic::solve_quadratic;

pub fn quadratic_benchmark(c: &mut Criterion) {
    c.bench_function("Quadratic 1", |b| {
        b.iter(|| solve_quadratic(black_box(1.0), black_box(-1.0), black_box(-2.0)))
    });
}

criterion_group!(benches, quadratic_benchmark);
criterion_main!(benches);
