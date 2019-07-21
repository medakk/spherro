extern crate spherro;


// This is just a binary target for easy profiling with perf.
// Ideally I'd use cargo bench to generate this binary...
fn main() {
    let config = spherro::Config::new(0.4, 0.8, 50, 10);
    let mut universe = spherro::Universe::new(
        600.0, 600.0, &config,
    );

    for _ in 0..10000 {
        universe.update(0.001)
    }
}