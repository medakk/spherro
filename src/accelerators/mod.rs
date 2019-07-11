use crate::util::{Vector2f};

pub trait HasPosition {
    fn position(&self) -> Vector2f;
}

pub trait Accelerator {
    fn nearest_by_idx(&self, i: usize, r: f32) -> Vec<usize>;
    fn nearest_by_pos(&self, pos: Vector2f, r: f32) -> Vec<usize>;
}

mod grid;

pub use grid::Grid;