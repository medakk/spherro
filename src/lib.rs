#[macro_use]
#[allow(unused_macros)]
#[allow(dead_code)]

extern crate itertools;

#[macro_use]
pub mod util; //TODO: make this private

#[allow(dead_code)]
mod accelerators;
mod particle;
mod universe;
mod kernel;
mod fetcher;
pub mod initializer;

// Re-export some names for flatter syntax
pub use particle::Particle;
pub use universe::{Universe};
pub use fetcher::{Fetcher};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;