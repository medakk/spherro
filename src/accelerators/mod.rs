use crate::util::{Vector3f};

pub trait HasPosition {
    fn position(&self) -> Vector3f;
}

pub trait Accelerator {
    fn nearest_neighbours(&self, i: usize, r: f32) -> Vec<usize>;
}

mod quadtree;
mod grid;

pub use quadtree::Quadtree;
pub use grid::Grid;