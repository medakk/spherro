#[macro_use]
#[allow(unused_macros)]
#[allow(dead_code)]

extern crate itertools;

#[allow(dead_code)]
mod accelerators;
mod particle;
mod universe;
mod kernel;
mod blob;
pub mod initializer;
pub mod util; //TODO: make this private

// Re-export some names for flatter syntax
pub use particle::Particle;
pub use universe::{Universe};
pub use blob::Blob;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;