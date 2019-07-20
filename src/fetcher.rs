use wasm_bindgen::prelude::*;
use crate::universe::Universe;
use noisy_float::prelude::*;

// 2 floats for position
// 2 floats for velocity
const STRIDE: usize = 4;

// Fetches data from the universe into a buffer
// for wasm to read. The point of this is to separate
// the Universe's concern from the data format needed
// by the client
pub struct Fetcher {
    buffer: Vec<R32>,
}

#[wasm_bindgen]
impl Fetcher {
    pub fn new() -> Fetcher {
        Fetcher {
            buffer: Vec::new(),
        }
    }

    pub fn fetch(&mut self, universe: &Universe) -> *const R32 {
        self.buffer.resize(universe.get_size() * STRIDE, r32(0.0));

        for (i, pi) in universe.get_particles().iter().enumerate() {
            self.buffer[i*STRIDE + 0] = pi.pos.x;
            self.buffer[i*STRIDE + 1] = pi.pos.y;
            self.buffer[i*STRIDE + 2] = pi.vel.x;
            self.buffer[i*STRIDE + 3] = pi.vel.y;
        }

        self.buffer.as_ptr()
    }

    pub fn stride(&self) -> usize {
        STRIDE
    }
}