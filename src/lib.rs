#[macro_use]
#[allow(unused_macros)]
#[allow(dead_code)]
mod util;

#[macro_use]
extern crate itertools;

mod quadtree;
mod particle;
mod universe;
pub mod initializer;
mod kernel;

// Re-export some names for flatter syntax
pub use particle::Particle;
pub use universe::{Universe};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;