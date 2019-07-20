use wasm_bindgen::prelude::*;
use crate::universe::Universe;

// 2 floats for position
// 2 floats for velocity
// 3 float for color
const STRIDE: usize = 7;

// Fetches data from the universe into a buffer
// for wasm to read. The point of this is to separate
// the Universe's concern from the data format needed
// by the client
#[wasm_bindgen]
pub struct Fetcher {
    buffer: Vec<f32>,
}

#[wasm_bindgen]
impl Fetcher {
    pub fn new() -> Fetcher {
        Fetcher {
            buffer: Vec::new(),
        }
    }

    pub fn fetch(&mut self, universe: &Universe) -> *const f32 {
        self.buffer.resize(universe.get_size() * STRIDE, 0.0);

        for (i, pi) in universe.get_particles().iter().enumerate() {
            self.buffer[i*STRIDE + 0] = pi.pos.x;
            self.buffer[i*STRIDE + 1] = pi.pos.y;
            self.buffer[i*STRIDE + 2] = pi.vel.x;
            self.buffer[i*STRIDE + 3] = pi.vel.y;
            self.buffer[i*STRIDE + 4] = pi.col.x;
            self.buffer[i*STRIDE + 5] = pi.col.y;
            self.buffer[i*STRIDE + 6] = pi.col.z;
        }

        self.buffer.as_ptr()
    }

    pub fn stride(&self) -> usize {
        STRIDE
    }
}