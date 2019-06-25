mod utils;
mod vector3;

use wasm_bindgen::prelude::*;
use vector3::*;
use utils::*;

extern crate js_sys;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[repr(C)]
pub struct Particle {
    pos: Vector3,
    mass: f32,
    density: f32,
    pressure: f32,
}

impl Particle {
    pub fn new(pos: Vector3, mass: f32, density: f32, pressure: f32) -> Particle {
        Particle{pos, mass, density, pressure}
    }
}

#[wasm_bindgen]
pub struct Universe {
    particles: Vec<Particle>,
    width: usize,
    height: usize,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: usize, height: usize) -> Universe {
        let mut particles = Vec::new();

        for _ in 0..100 {
            let x: f32 = (js_sys::Math::random() as f32) * width as f32;
            let y: f32 = (js_sys::Math::random() as f32) * height as f32;

            let position = Vector3::new(x, y, 0.0);
            particles.push(Particle::new(position, 1.0, 1.0, 1.0));
        }

        Universe {
            particles,
            width,
            height,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let d = Vector3::new(0.0, 100.0, 0.0).scale(dt);
        let new_particles: Vec<_> = self.particles.iter().map(|p| {
            let mut new_pos = p.pos.add(&d);
            new_pos.x = clamp_f32(new_pos.x, 0.0, self.width as f32);
            new_pos.y = clamp_f32(new_pos.y, 0.0, self.height as f32);

            Particle::new(new_pos, p.mass, p.density, p.pressure)
        }).collect();

        self.particles = new_particles;
    }

    pub fn get_particle_positions(&self) -> *const Particle {
        self.particles.as_ptr()
    }

    pub fn get_size(&self) -> usize {
        self.particles.len()
    }
}

impl Universe {
    fn get_neighbours(&self, pi: usize) -> Vec<&Particle>{
        let mut neighbours: Vec<&Particle> = Vec::new();

        let p1 = &self.particles[pi];
        for i in 0..self.get_size() {
            if i==pi {
                continue;
            }

            let p2 = &self.particles[i];
            let dist = p1.pos.distance_to(&p2.pos);

            if dist < 10.0 {
                neighbours.push(p2);
            }
        }

        neighbours
    }
}