use wasm_bindgen::prelude::*;
use crate::Universe;
use crate::util::{clamp_f32};

// This exists to serve as a way for the universe to write out its contents
#[wasm_bindgen]
pub struct Blob {
    width: usize,
    height: usize,
    image: Vec<u8>,
}

#[wasm_bindgen]
impl Blob {
    pub fn new(width: usize, height: usize) -> Blob {
        let image = vec![0u8; width*height];
        Blob {
            width: width,
            height: height,
            image: image,
        }
    }

    pub fn test_things(&mut self) {
        for i in 0..self.width*self.height {
            if self.image[i] == 255 {
                self.image[i] = 0;
            } else {
                self.image[i] += 1;
            }
        }
    }

    pub fn set_from_universe(&mut self, universe: &Universe) {
        for i in 0..self.image.len() {
            self.image[i] = 0;
        }

        let w = universe.get_width();
        let h = universe.get_height();

        for pi in universe.get_particles() {
            let x = clamp_f32(pi.pos.x / w, 0.0, 1.0);
            let x = (x * self.width as f32) as usize;

            let y = clamp_f32(pi.pos.y / h, 0.0, 1.0);
            let y = (y * self.height as f32) as usize;

            self.image[y * self.width + x] = 255;
        }
    }

    pub fn get_data(&self) -> *const u8 {
        self.image.as_ptr()
    }
}