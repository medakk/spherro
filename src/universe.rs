use wasm_bindgen::prelude::*;
use cgmath::{InnerSpace, VectorSpace};
use crate::util::*;
use crate::particle::{Particle};
use crate::accelerators::{Accelerator, Grid};
use crate::initializer;
use crate::kernel::*;

const H: f32 = 35.0;
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
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: f32, height: f32, strategy: initializer::Strategy) -> Universe {
        set_panic_hook();
        let particles = initializer::initialize(width, height, strategy);

        Universe {
            particles: particles,
            width: width,
            height: height,
            neighbours: Vec::new(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        let accel = Grid::new(self.width, self.height, H, &self.particles);
        self.neighbours = (0..self.particles.len()).map(|i| {
            accel.nearest_neighbours(i, H*2.0)
        }).collect();

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
        const COL_BLUE: Color = Color::new(0.0, 0.0, 1.0);
        const COL_RED: Color = Color::new(1.0, 0.0, 0.0);

        self.particles.iter().enumerate().map(|(i, pi)| {
            let rho: f32 = self.neighbours[i].iter().map(|&j| {
                let pj = &self.particles[j];
                let x_ij = pi.pos - pj.pos;
                let q = x_ij.magnitude() / H;
                let Wj = cubicspline_f(q) / H.powi(3);
                pj.mass * Wj
            }).sum();

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
        self.particles.iter().enumerate().map(|(i, pi)| {
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

    fn compute_dv(&self, pi: &Particle, neighbours: Vec<&Particle>) -> Vector2f {
        // Compute x_ijs
        let x_ijs: Vec<Vector2f> = neighbours.iter().map(|pj| {
            pi.pos - pj.pos
        }).collect();

        // Compute gradient of W
        let dWs: Vec<Vector2f> = izip!(&neighbours, &x_ijs).map(|(pj, x_ij)| {
            let q = x_ij.magnitude() / H;
            let df = cubicspline_df(q);

            let dq = (pi.pos - pj.pos) / (H * q); // gradient of q
            let dW = (1.0 / H.powi(3)) * df * dq;

            dW
        }).collect();


        // Compute gradient of pressure
        let dP = pi.rho * izip!(&neighbours, &dWs).map(|(pj, dW)| {
            pj.mass * (pi.pressure / pi.rho.powi(2) + pj.pressure / pj.rho.powi(2)) * dW
        }).sum::<Vector2f>();

        // Compute laplacian of velocities
        let ddv = 2.0 * izip!(&neighbours, &x_ijs, &dWs).map(|(pj, x_ij, dW)| {
            let q1 = (pj.mass / pj.rho) * (pi.vel - pj.vel);
            let q2 = (x_ij.dot(*dW)) / (x_ij.dot(*x_ij) + 0.01*H*H);
            q1 * q2
        }).sum::<Vector2f>();

        // Accceleration due to gravity
        let gravity = Vector2f::new(0.0, GRAVITY);

        let dv = (-1.0 / pi.rho) * dP + VISC * ddv + gravity;

        dv
    }

    fn compute_wall_bounce(&self, pos: &Vector2f, vel: &Vector2f) -> Vector2f {
        const B: f32 = 0.9;
        let mut vel: Vector2f = *vel;

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
        const CHOSEN_IDX: usize = 247;

        let accel = Grid::new(self.width, self.height, H, &self.particles);
        let neighbours = accel.nearest_neighbours(CHOSEN_IDX, H*2.0);
        self.particles[CHOSEN_IDX].col = Color::new(0.0, 0.0, 0.0);
        for j in neighbours.into_iter() {
            self.particles[j].col = Color::new(1.0, 1.0, 0.0);
        }
    }

    pub fn debug_check_nans(&self) {
        let mut is_bad = false;
        for (i, pi) in self.particles.iter().enumerate() {
            if !pi.pos.x.is_finite() || !pi.pos.y.is_finite() {

                let accel = Grid::new(self.width, self.height, H, &self.particles);
                let neighbours = accel.nearest_neighbours(i, H*2.0);

                println!("Found bad particle with idx {}: {:?}\nNeighbour count: {}", i, pi, neighbours.len());
                is_bad = true;
            }
        }

        if is_bad {
            panic!();
        }
    }

    pub fn debug_splits(&self) -> Vec<(Vector2f, Vector2f)> {
        let accel = Grid::new(self.width, self.height, H*2.0, &self.particles);
        accel.debug_get_splits()
    }

    pub fn clear_colors(&mut self) {
        for pi in self.particles.iter_mut() {
            pi.col = Color::new(0.0, 0.0, 1.0);
        }
    }
}