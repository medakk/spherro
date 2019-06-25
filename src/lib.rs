mod utils;
mod vector3;

use wasm_bindgen::prelude::*;
use vector3::*;

extern crate js_sys;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Universe {
    particles: Vec<Vector3>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let mut particles = Vec::new();

        for i in 1..100 {
            let x: f32 = (js_sys::Math::random() as f32) * 700.0;
            let y: f32 = (js_sys::Math::random() as f32) * 700.0;
            particles.push(Vector3::new(x, y, 0.0));
        }

        Universe {
            particles,
        }
    }

    pub fn update(&mut self) {
        let d = Vector3::new(0.0, 1.0, 0.0);
        let new_particles: Vec<_> = self.particles.iter().map(|v| {
            v.add(&d)
        }).collect();

        self.particles = new_particles;
    }

    pub fn get_particle_positions(&self) -> *const Vector3 {
        self.particles.as_ptr()
    }

    pub fn get_size(&self) -> usize {
        self.particles.len()
    }
}