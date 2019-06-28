#[macro_use]
mod utils;

extern crate js_sys;
extern crate cgmath;

use wasm_bindgen::prelude::*;

use utils::*;

use cgmath::{MetricSpace};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const H: f32 = 30.0;
const REST_RHO: f32 = 1.0 / (10.0 * 10.0 * 10.0);

#[repr(C)]
pub struct Particle {
    pos: Vector3f,
    vel: Vector3f,
    mass: f32,
    rho: f32,
    pressure: f32,
}

impl Particle {
    pub fn new(pos: Vector3f, vel: Vector3f, mass: f32, rho: f32, pressure: f32) -> Particle {
        Particle{pos, vel, mass, rho, pressure}
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

        for _ in 0..500 {
            let x: f32 = (js_sys::Math::random() as f32) * width as f32;
            let y: f32 = (js_sys::Math::random() as f32) * height as f32;

            let position = Vector3f::new(x, y, 0.0);
            particles.push(Particle::new(position, vec3f_zero(), 1.0, 1.0, 1.0));
        }

        Universe {
            particles,
            width,
            height,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.particles = self.updated_particle_fields();

        let new_particles: Vec<_> = self.particles.iter().enumerate().map(|(pi, p)| {
            // Compute Navier stokes
            let neighbours = self.get_neighbours(pi);
            let mut dv = self.compute_dv(p, neighbours);

            // Gravity
            dv.y += 100.0;

            // Find velocity
            let mut vel = p.vel + dv * dt;

            // Find position
            let pos = p.pos + vel * dt;

            // Bounce off walls
            if pos.x < 0.0 || pos.x > self.width as f32 {
                vel.x *= -0.90;
            }
            if pos.y < 0.0 || pos.y > self.height as f32 {
                vel.y *= -0.90;
            }

            Particle::new(pos, vel, p.mass, p.rho, p.pressure)
        }).collect();

        self.particles = new_particles;
    }

    pub fn get_data_stride(&self) -> usize {
        std::mem::size_of::<Particle>()
    }

    pub fn get_data(&self) -> *const Particle {
        self.particles.as_ptr()
    }

    pub fn get_size(&self) -> usize {
        self.particles.len()
    }
}

#[allow(non_snake_case)]
impl Universe {
    fn get_neighbours(&self, pi: usize) -> Vec<&Particle>{
        let mut neighbours: Vec<&Particle> = Vec::new();

        let p1 = &self.particles[pi];
        for i in 0..self.get_size() {
            if i==pi {
                continue;
            }

            let p2 = &self.particles[i];
            let dist = p1.pos.distance(p2.pos);

            if dist < H * 2.5 {
                neighbours.push(p2);
            }
        }

        neighbours
    }

    fn updated_particle_fields(&self) -> Vec<Particle> {
        let mut new_particles: Vec<Particle> = Vec::new();

        for (i, pi) in self.particles.iter().enumerate() {
            let neighbours = self.get_neighbours(i);

            let x_ijs: Vec<f32> = neighbours.iter().map(|pj| {
                pi.pos.distance(pj.pos)
            }).collect();

            let W: Vec<f32> = x_ijs.iter().map(|x_ij| {
                let q = x_ij / H;
                cubic_spline(q).0 / H.powf(3.0)
            }).collect();

            let mut rho: f32 = 0.0;
            for (j, pj) in neighbours.iter().enumerate() {
                rho += pj.mass * W[j]
            }

            let pressure = 1.0 * ((rho / REST_RHO).powf(7.0) - 1.0);

            new_particles.push(Particle::new(
                pi.pos, pi.vel, pi.mass, rho, pressure
            ));
        }

        new_particles
    }

    fn compute_dv(&self, pi: &Particle, neighbours: Vec<&Particle>) -> Vector3f {
        // General: Compute x_ijs
        let x_ijs: Vec<f32> = neighbours.iter().map(|p_j| {
            pi.pos.distance(p_j.pos)
        }).collect();

        let dWs: Vec<Vector3f> = neighbours.iter().zip(x_ijs).map(|(pj, x_ij)| {
            let q = x_ij / H;
            let (_f, df) = cubic_spline(q);

            // Derivative of q wrt x, y and z
            let dq_x = (pi.pos.x - pj.pos.x) / (H * q);
            let dq_y = (pi.pos.y - pj.pos.y) / (H * q);
            let dq_z = (pi.pos.z - pj.pos.z) / (H * q);
            
            let dW_x = (1.0 / H.powf(3.0)) * df * dq_x;
            let dW_y = (1.0 / H.powf(3.0)) * df * dq_y;
            let dW_z = (1.0 / H.powf(3.0)) * df * dq_z;

            Vector3f::new(dW_x, dW_y, dW_z)
        }).collect();

        let dP: Vector3f = neighbours.iter().zip(dWs).map(|(pj, dW)| {
            (pj.mass / pj.rho) * pj.pressure * dW
        }).sum();

        let dv = (-1.0 / pi.rho) * dP;

        dv
    }
}

// Returns the value and gradient of the value
fn cubic_spline(q: f32) -> (f32, f32) {
    if 0.0 <= q && q < 1.0 {
        let v = (2.0/3.0) - (q*q) + (0.5*q*q*q);
        let dv = (-2.0 * q) + (1.5 * q*q);
        (v, dv)
    } else if 1.0 <= q && q < 2.0 {
        let v = (1.0/6.0) * (2.0 - q).powf(3.0);
        let dv = -0.5 * (2.0 - q).powf(2.0);
        (v, dv)
    } else {
        (0.0, 0.0)
    }
}