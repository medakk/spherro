#[macro_use]
extern crate criterion;
extern crate spherro;

use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
    let mut universe = spherro::Universe::new(
        600.0, 600.0, spherro::initializer::Strategy::DAMBREAK,
    );

    c.bench_function("solver_step 0.001", move |b| b.iter(|| universe.update(0.001)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);