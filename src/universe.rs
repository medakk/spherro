use wasm_bindgen::prelude::*;
use cgmath::{InnerSpace, VectorSpace};
use rayon::prelude::*;
use crate::util::*;
use crate::particle::{Particle};
use crate::accelerators::{Accelerator, Grid};
use crate::initializer;
use crate::kernel::{Kernel, CubicSpline};

const H: f32 = 30.0;
const VISC: f32 = 10.0;
const REST_RHO: f32 = 1.0 / (5.0 * 5.0 * 5.0);
const BOUNCE_MIN_DV: f32 = 500.0;
const GRAVITY: f32 = -10000.0;
const K: f32 = 10.0;

#[wasm_bindgen]
pub struct Universe {
    particles: Vec<Particle>,
    width: f32,
    height: f32,
    neighbours: Vec<Vec<usize>>,
    kernel: Box<Kernel + std::marker::Sync>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: f32, height: f32, strategy: initializer::Strategy) -> Universe {
        let particles = initializer::initialize(width, height, strategy);
        let kernel = CubicSpline{};

        Universe {
            particles: particles,
            width: width,
            height: height,
            neighbours: Vec::new(),
            kernel: Box::new(kernel),
        }
    }

    pub fn update(&mut self, dt: f32) {
        let accel = Grid::new(self.width, self.height, H*2.0, &self.particles);
        self.neighbours = (0..self.particles.len()).into_par_iter().map(|i| {
            accel.nearest_neighbours(i, H*2.0)
        }).collect();

        //TODO: Figure out how to update these things in place
        self.particles = self.updated_particle_fields(dt);
        self.particles = self.updated_particle_positions(dt);
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

    fn updated_particle_fields(&self, _dt: f32) -> Vec<Particle> {
        const COL_BLUE: Vector3f = Vector3f::new(0.0, 0.0, 1.0);
        const COL_RED: Vector3f = Vector3f::new(1.0, 0.0, 0.0);

        self.particles.par_iter().enumerate().map(|(i, pi)| {
            let neighbours: Vec<&Particle> = self.neighbours[i]
                                             .iter()
                                             .map(|&j| { &self.particles[j] })
                                             .collect();

            let x_ijs: Vec<Vector3f> = neighbours.iter().map(|&pj| {
                pi.pos - pj.pos
            }).collect();

            let W: Vec<f32> = x_ijs.iter().map(|x_ij| {
                let q = x_ij.magnitude() / H;
                (*self.kernel).f(q) / H.powi(3)
            }).collect();

            let rho: f32 = izip!(neighbours, &W)
                          .map(|(pj, Wj)| {
                              pj.mass * Wj
                           })
                          .sum();
            let pressure = K * ((rho / REST_RHO).powi(7) - 1.0);
            let col = COL_BLUE.lerp(COL_RED, rho / REST_RHO);

            Particle{
                col: col,
                rho: rho,
                pressure: pressure,
                ..*pi
            }
        }).collect()
    }

    fn updated_particle_positions(&self, dt: f32) -> Vec<Particle> {
        self.particles.par_iter().enumerate().map(|(i, pi)| {
            let neighbours: Vec<&Particle> = self.neighbours[i]
                                             .iter()
                                             .map(|&j| { &self.particles[j] })
                                             .collect();

            // Compute navier stokes update
            let dv = self.compute_dv(pi, neighbours);

            // Find velocity
            let mut vel = pi.vel + dv * dt;

            // Find position
            let pos = pi.pos + vel * dt;

            // Wall bounce update
            vel = self.compute_wall_bounce(&pos, &vel);

            Particle{
                pos: pos,
                vel: vel,
                ..*pi
            }
        }).collect()
    }

    fn compute_dv(&self, pi: &Particle, neighbours: Vec<&Particle>) -> Vector3f {
        // Compute x_ijs
        let x_ijs: Vec<Vector3f> = neighbours.iter().map(|pj| {
            pi.pos - pj.pos
        }).collect();

        // Compute gradient of W
        let dWs: Vec<Vector3f> = izip!(&neighbours, &x_ijs).map(|(pj, x_ij)| {
            let q = x_ij.magnitude() / H;
            let df = (*self.kernel).df(q);

            let dq = (pi.pos - pj.pos) / (H * q); // gradient of q
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

    fn compute_wall_bounce(&self, pos: &Vector3f, vel: &Vector3f) -> Vector3f {
        const B: f32 = 0.9;
        let mut vel: Vector3f = *vel;

        // Bounce off walls
        if pos.x < 0.0 {
            vel.x = (BOUNCE_MIN_DV).max(-B*vel.x);
        } else if pos.x > self.width {
            vel.x = (-BOUNCE_MIN_DV).min(-B*vel.x);
        } else if pos.y < 0.0 {
            vel.y = (BOUNCE_MIN_DV).max(-B*vel.y);
        } else if pos.y > self.height {
            vel.y = (-BOUNCE_MIN_DV).min(-B*vel.y);
        }

        vel
    }
}

// All debug functions
impl Universe {
    pub fn debug_update(&mut self, _dt: f32) {
        let accel = Grid::new(self.width, self.height, H*2.0, &self.particles);
        let neighbours = self.get_neighbour_indices(0, &accel);
        self.particles[0].col = vec3f_zero();
        for j in neighbours.into_iter() {
            self.particles[j].col = Vector3f::new(1.0, 1.0, 0.0);
        }
    }
    
    fn get_neighbour_indices(&self, i: usize, accel: &Accelerator) -> Vec<usize> {
        let neighbour_indices = accel.nearest_neighbours(i, H*2.0);
        neighbour_indices
    }

    pub fn debug_splits(&self) -> Vec<(Vector3f, Vector3f)> {
        // let accel = Quadtree::new(self.width, self.height, &self.particles);
        let accel = Grid::new(self.width, self.height, H*2.0, &self.particles);
        accel.debug_get_splits()
    }

    pub fn clear_colors(&mut self) {
        for pi in self.particles.iter_mut() {
            pi.col = Vector3f::new(0.0, 0.0, 1.0);
        }
    }
}