use wasm_bindgen::prelude::*;
use cgmath::{InnerSpace, VectorSpace};
use rand::Rng;
use crate::util::*;
use crate::particle::{Particle};
use crate::octree::{Octree};

const H: f32 = 30.0;
const VISC: f32 = 10.0;
const REST_RHO: f32 = 1.0 / (5.0 * 5.0 * 5.0);
const BOUNCE_MIN_DV: f32 = 1000.0;
const GRAVITY: f32 = -10000.0;
const K: f32 = 10.0;

#[wasm_bindgen]
pub struct Universe {
    particles: Vec<Particle>,
    width: f32,
    height: f32,
}

pub enum InitializerStrategy {
    RANDOM,
    DAMBREAK,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: f32, height: f32, strategy: InitializerStrategy) -> Universe {
        let particles = match strategy {
            InitializerStrategy::RANDOM => {
                let mut rng = rand::thread_rng();
                let mut particles = Vec::new();

                for _ in 0..500 {
                    let x: f32 = rng.gen::<f32>() * width;
                    let y: f32 = rng.gen::<f32>() * height;

                    let position = Vector3f::new(x, y, 0.0);
                    let col = Vector3f::new(0.0, 0.0, 1.0);
                    particles.push(Particle::new(position, col, vec3f_zero(), 100.0, 1.0, 1.0));
                }

                particles
            },
            InitializerStrategy::DAMBREAK => {
                let mut particles = Vec::new();

                let width = 0.40 * width;
                let height = 0.80 * height;

                let rows = 50.0;
                let cols = 10.0;

                let x_spacing = (width / cols) as usize;
                let y_spacing = (height / rows) as usize;

                for i in 0..cols as usize {
                    for j in 0..rows as usize {
                        let x = (x_spacing * (i + (j % 2)*3)) as f32;
                        let y = (y_spacing * j) as f32;

                        let position = Vector3f::new(x, y, 0.0);
                        let col = Vector3f::new(0.0, 0.0, 1.0);
                        particles.push(Particle::new(position, col, vec3f_zero(), 100.0, 1.0, 1.0));
                    }
                }

                particles
            }
        };

        Universe {
            particles,
            width,
            height,
        }
    }

    pub fn update(&mut self, dt: f32) {
        //TODO: This function creates a lot of copies of particles
        let orig_particles = self.particles.clone();

        //TODO: Abstract out the accelerator. This will make it easier to
        // try different implementations
        let accel = Octree::new(self.width, self.height, &orig_particles);

        //TODO: Compute neighbours once, and just pass that into the update methods

        self.particles = self.updated_particle_fields(&accel);

        //TODO: refactor this into a function
        let new_particles: Vec<_> = self.particles.iter().enumerate().map(|(pi, p)| {
            // Compute Navier stokes
            let neighbours = self.get_neighbours(pi, &accel);
            let dv = self.compute_dv(p, neighbours);

            // Find velocity
            let mut vel = p.vel + dv * dt;

            // Find position
            let pos = p.pos + vel * dt;

            const B: f32 = 0.9;
            // Bounce off walls
            if pos.x < 0.0 {
                vel.x = max_f32(-B*vel.x, BOUNCE_MIN_DV);
            } else if pos.x > self.width {
                vel.x = min_f32(-B*vel.x, -BOUNCE_MIN_DV);
            } else if pos.y < 0.0 {
                vel.y = max_f32(-B*vel.y, BOUNCE_MIN_DV);
            } else if pos.y > self.height {
                vel.y = min_f32(-B*vel.y, -BOUNCE_MIN_DV);
            }

            Particle::new(pos, p.col, vel, p.mass, p.rho, p.pressure)
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

    pub fn get_particles(&self) -> &Vec<Particle> {
        &self.particles
    }

    fn get_neighbours(&self, i: usize, accel: &Octree<Particle>) -> Vec<&Particle>{
        self.get_neighbour_indices(i, accel).into_iter().map(|j| {
            &self.particles[j]
        }).collect()
    }

    fn get_neighbour_indices(&self, i: usize, accel: &Octree<Particle>) -> Vec<usize> {
        let neighbour_indices = accel.nearest_neighbours_indices(i, H*2.0);
        neighbour_indices
    }

    fn updated_particle_fields(&self, accel: &Octree<Particle>) -> Vec<Particle> {
        let mut new_particles: Vec<Particle> = Vec::new();

        const COL_BLUE: Vector3f = Vector3f::new(0.0, 0.0, 1.0);
        const COL_RED: Vector3f = Vector3f::new(1.0, 0.0, 0.0);

        for (i, pi) in self.particles.iter().enumerate() {
            let neighbours = self.get_neighbours(i, accel);

            let x_ijs: Vec<Vector3f> = neighbours.iter().map(|pj| {
                pi.pos - pj.pos
            }).collect();

            let W: Vec<f32> = x_ijs.iter().map(|x_ij| {
                let q = x_ij.magnitude() / H;
                cubic_spline(q).0 / H.powi(3)
            }).collect();

            let mut rho: f32 = 0.0;
            for (j, pj) in neighbours.iter().enumerate() {
                rho += pj.mass * W[j]
            }

            let pressure = K * ((rho / REST_RHO).powi(7) - 1.0);

            let col = COL_BLUE.lerp(COL_RED, rho / REST_RHO);

            new_particles.push(Particle::new(
                pi.pos, col, pi.vel, pi.mass, rho, pressure
            ));
        }

        new_particles
    }

    fn compute_dv(&self, pi: &Particle, neighbours: Vec<&Particle>) -> Vector3f {
        // Compute x_ijs
        let x_ijs: Vec<Vector3f> = neighbours.iter().map(|pj| {
            pi.pos - pj.pos
        }).collect();

        // Compute gradient of W
        let dWs: Vec<Vector3f> = izip!(&neighbours, &x_ijs).map(|(pj, x_ij)| {
            let q = x_ij.magnitude() / H;
            let (_f, df) = cubic_spline(q);

            // Derivative of q wrt x, y and z
            let dq = (pi.pos - pj.pos) / (H * q);
            let dW = (1.0 / H.powi(3)) * df * dq;

            dW
        }).collect();


        // Compute gradient of pressure
        let dP: Vector3f = pi.rho * izip!(&neighbours, &dWs).map(|(pj, dW)| {
            pj.mass * (pi.pressure / pi.rho.powi(2) + pj.pressure / pj.rho.powi(2)) * dW
        }).sum::<Vector3f>();

        // Compute laplacian of velocities
        let ddv = 2.0 * izip!(&neighbours, &x_ijs, &dWs).map(|(pj, x_ij, dW)| {
            let q1 = (pj.mass / pj.rho) * (pi.vel - pj.vel);
            let q2 = (x_ij.dot(*dW)) / (x_ij.dot(*x_ij) + 0.01*H*H);
            q1 * q2
        }).sum::<Vector3f>();

        // Accceleration due to gravity
        let gravity = Vector3f::new(0.0, GRAVITY, 0.0);

        let dv = (-1.0 / pi.rho) * dP + VISC * ddv + gravity;

        dv
    }
}

//TODO: move this elsewhere, make a different function for the gradient
// and maybe more generic to allow plugging in different functions here
// Returns the value and gradient of the value
fn cubic_spline(q: f32) -> (f32, f32) {
    if 0.0 <= q && q < 1.0 {
        let v = (2.0/3.0) - (q*q) + (0.5*q*q*q);
        let dv = (-2.0 * q) + (1.5 * q*q);
        (v, dv)
    } else if 1.0 <= q && q < 2.0 {
        let v = (1.0/6.0) * (2.0 - q).powi(3);
        let dv = -0.5 * (2.0 - q).powi(2);
        (v, dv)
    } else {
        (0.0, 0.0)
    }
}

impl Universe {
    // All debug functions

    pub fn debug_update(&mut self, _dt: f32) {
        let accel = Octree::new(self.width, self.height, &self.particles);
        let neighbours = self.get_neighbour_indices(0, &accel);
        self.particles[0].col = vec3f_zero();
        for j in neighbours.into_iter() {
            self.particles[j].col = Vector3f::new(1.0, 1.0, 0.0);
        }
    }

    pub fn debug_splits(&self) -> Vec<(Vector3f, Vector3f)> {
        let accel = Octree::new(self.width, self.height, &self.particles);
        accel.debug_get_splits()
    }

    pub fn clear_colors(&mut self) {
        for pi in self.particles.iter_mut() {
            pi.col = Vector3f::new(0.0, 0.0, 1.0);
        }
    }
}