use crate::accelerators::{HasPosition, Accelerator};
use crate::util::*;
use cgmath::{InnerSpace};

pub struct Grid<'a, T> {
    width: f32,
    height: f32,
    bin_size: f32,
    cells: Vec<Cell>,
    items: &'a [T],
}

struct Cell {
    items: Vec<usize>,
}

impl<'a, T> Accelerator for Grid<'a, T> where T: HasPosition {
    fn nearest_by_idx(&self, i: usize, r: f32) -> Vec<usize> {
        let pos = self.items[i].position();
        self.nearest(pos, r, Some(i))
    }

    fn nearest_by_pos(&self, pos: Vector2f, r: f32) -> Vec<usize> {
        self.nearest(pos, r, None)
    }
}

impl<'a, T> Grid<'a, T> where T: HasPosition {
    pub fn new(width: f32, height: f32, bin_size: f32, items: &'a [T]) -> Self {
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
    fn nearest(&self, pos: Vector2f, r: f32, filter_idx: Option<usize>) -> Vec<usize> {
        let cols = (self.width / self.bin_size).ceil() as usize;
        let _rows = (self.height / self.bin_size).ceil() as usize;

        // We save some time on allocation by preallocating space.
        // Maybe also compute this heurestic on the fly after seeing a few samples.
        let mut neighbours = Vec::with_capacity(24);

        let x0 = clamp_f32(pos.x - r, 0.0, self.width  as f32 - 1e-2);
        let x1 = clamp_f32(pos.x + r, 0.0, self.width  as f32 - 1e-2);
        let y0 = clamp_f32(pos.y - r, 0.0, self.height as f32 - 1e-2);
        let y1 = clamp_f32(pos.y + r, 0.0, self.height as f32 - 1e-2);

        let x0 = (x0 / self.bin_size) as usize;
        let x1 = (x1 / self.bin_size) as usize;
        let y0 = (y0 / self.bin_size) as usize;
        let y1 = (y1 / self.bin_size) as usize;

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

    fn construct_grid(width: f32, height: f32, bin_size: f32, items: &'a [T]) -> Vec<Cell> {
        let cols = (width / bin_size).ceil() as usize;
        let rows = (height / bin_size).ceil() as usize;
        let n_cells = cols * rows;

        let mut cells: Vec<Cell> = (0..n_cells).map(|_i| {
            Cell{items:Vec::new()}
        }).collect();

        for (i, pi) in items.iter().enumerate() {
            let pos = pi.position();

            let x = clamp_f32(pos.x, 0.0, width-1e-2);
            let y = clamp_f32(pos.y, 0.0, height-1e-2);
            let x = (x / bin_size) as usize;
            let y = (y / bin_size) as usize;

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
                Vector2f::new(x as f32 * self.bin_size, 0.0),
                Vector2f::new(x as f32 * self.bin_size, self.height),
            ));
        }

        for y in 0..rows {
            splits.push((
                Vector2f::new(0.0, y as f32 * self.bin_size),
                Vector2f::new(self.width, y as f32 * self.bin_size),
            ));
        }

        splits
    }

}