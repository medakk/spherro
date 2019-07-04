extern crate spherro;


// This is just a binary target for easy profiling with perf.
// Ideally I'd use cargo bench to generate this binary...
fn main() {
    let mut universe = spherro::Universe::new(
        600.0, 600.0, spherro::initializer::Strategy::DAMBREAK,
    );

    for _ in 0..10000 {
        universe.update(0.001)
    }
}