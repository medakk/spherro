use crate::accelerators::{HasPosition, Accelerator};
use crate::util::*;
use noisy_float::prelude::*;
use num_traits::cast::ToPrimitive;
use cgmath::{InnerSpace};

pub struct Grid<'a, T> {
    width: R32,
    height: R32,
    bin_size: R32,
    cells: Vec<Cell>,
    items: &'a [T],
}

struct Cell {
    items: Vec<usize>,
}

impl<'a, T> Accelerator for Grid<'a, T> where T: HasPosition {
    fn nearest_by_idx(&self, i: usize, r: R32) -> Vec<usize> {
        let pos = self.items[i].position();
        self.nearest(pos, r, Some(i))
    }

    fn nearest_by_pos(&self, pos: Vector2f, r: R32) -> Vec<usize> {
        self.nearest(pos, r, None)
    }
}

impl<'a, T> Grid<'a, T> where T: HasPosition {
    pub fn new(width: R32, height: R32, bin_size: R32, items: &'a [T]) -> Self {
        let cells = Grid::<T>::construct_grid(width, height, bin_size, items);

        Grid{
            width: width,
            height: height,
            bin_size: bin_size,
            cells: cells,
            items: items,
        }
    }

    // Returns the indices of the nearest items around pos within a radius of
    // `r`. If `filter_idx` is provided, that item is excluded in the returned indices
    fn nearest(&self, pos: Vector2f, r: R32, filter_idx: Option<usize>) -> Vec<usize> {
        let cols = (self.width / self.bin_size).ceil().to_usize().unwrap();
        let _rows = (self.height / self.bin_size).ceil().to_usize().unwrap();

        // We save some time on allocation by preallocating space.
        // Maybe also compute this heurestic on the fly after seeing a few samples.
        let mut neighbours = Vec::with_capacity(24);

        let x0 = clamp_R32(pos.x - r, r32(0.0), self.width  - 1e-2);
        let x1 = clamp_R32(pos.x + r, r32(0.0), self.width  - 1e-2);
        let y0 = clamp_R32(pos.y - r, r32(0.0), self.height - 1e-2);
        let y1 = clamp_R32(pos.y + r, r32(0.0), self.height - 1e-2);

        let x0 = (x0 / self.bin_size).floor().to_usize().unwrap();
        let x1 = (x1 / self.bin_size).floor().to_usize().unwrap();
        let y0 = (y0 / self.bin_size).floor().to_usize().unwrap();
        let y1 = (y1 / self.bin_size).floor().to_usize().unwrap();

        for x in x0..x1+1 {
            for y in y0..y1+1 {
                let idx = y * cols + x;
                for j in self.cells[idx].items.iter() {
                    let j = *j;

                    match filter_idx {
                        Some(i) if j == i => continue,
                        _ => {}
                    };

                    let pos_j = self.items[j].position();

                    if (pos - pos_j).magnitude2() < r*r {
                        neighbours.push(j);
                    }
                }
            }
        }

        neighbours
    }

    fn construct_grid(width: R32, height: R32, bin_size: R32, items: &'a [T]) -> Vec<Cell> {
        let cols = (width / bin_size).ceil() as usize;
        let rows = (height / bin_size).ceil() as usize;
        let n_cells = cols * rows;

        let mut cells: Vec<Cell> = (0..n_cells).map(|_i| {
            Cell{items:Vec::new()}
        }).collect();

        for (i, pi) in items.iter().enumerate() {
            let pos = pi.position();

            let x = clamp_R32(pos.x, 0.0, width-1e-2);
            let y = clamp_R32(pos.y, 0.0, height-1e-2);
            let x = (x / bin_size).floor() as usize;
            let y = (y / bin_size).floor() as usize;

            let idx = y * cols + x;
            cells[idx].items.push(i);
        }

        cells
    }

    pub fn debug_get_splits(&self) -> Vec<(Vector2f, Vector2f)> {
        let mut splits = Vec::new();
        let cols = (self.width / self.bin_size).ceil() as usize;
        let rows = (self.height / self.bin_size).ceil() as usize;

        for x in 0..cols {
            splits.push((
                Vector2f::new(x as R32 * self.bin_size, 0.0),
                Vector2f::new(x as R32 * self.bin_size, self.height),
            ));
        }

        for y in 0..rows {
            splits.push((
                Vector2f::new(0.0, y as R32 * self.bin_size),
                Vector2f::new(self.width, y as R32 * self.bin_size),
            ));
        }

        splits
    }

}