use wasm_bindgen::prelude::*;
use cgmath::{InnerSpace, VectorSpace};
use crate::util::*;
use crate::particle::{Particle};
use crate::accelerators::{Accelerator, Grid};
use crate::initializer;
use crate::kernel::*;
use crate::force::Force;

const MASS: f32 = 100.0;
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

    forces: Vec<Force>,
}

type Neighbours = Vec<Vec<usize>>;

#[wasm_bindgen]
impl Universe {
    pub fn new(width: f32, height: f32, strategy: initializer::Strategy) -> Universe {
        if cfg!(target_arch="wasm32") {
            set_panic_hook();
        }

        let particles = initializer::initialize(strategy, width, height, MASS);

        Universe {
            particles: particles,
            width: width,
            height: height,
            forces: Vec::new(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        let accel = Grid::new(self.width, self.height, H, &self.particles);

        let neighbours = (0..self.particles.len()).map(|i| {
            accel.nearest_by_idx(i, H*2.0)
        }).collect();

        let force_neighbours = self.forces.iter().map(|f| {
            accel.nearest_by_pos(f.pos(), f.r)
        }).collect();

        self.update_particle_fields(&neighbours, dt);
        self.update_navierstokes_dv(&neighbours, dt);
        self.update_forces_dv(&force_neighbours, dt);

        self.update_integrate(dt);
    }

    pub fn get_size(&self) -> usize {
        self.particles.len()
    }

    pub fn add_force(&mut self, force: Force) {
        self.forces.push(force);
    }

    pub fn clear_forces(&mut self) {
        self.forces.clear();
    }
}

#[allow(non_snake_case)]
impl Universe {

    pub fn get_particles(&self) -> &Vec<Particle> {
        &self.particles
    }

    // Updates the density, pressure and dv of every particle
    fn update_particle_fields(&mut self, neighbours: &Neighbours, _dt: f32) {
        const COL_BLUE: Color = Color::new(0.0, 0.0, 1.0);
        const COL_RED: Color = Color::new(1.0, 0.0, 0.0);

        for i in 0..self.particles.len() {
            let pi = &self.particles[i];
            let rho: f32 = neighbours[i].iter().map(|&j| {
                let pj = &self.particles[j];
                let x_ij = pi.pos - pj.pos;
                let q = x_ij.magnitude() / H;
                let Wj = cubicspline_f(q) / H.powi(3);
                pj.mass * Wj
            }).sum();

            let pressure = K * ((rho / REST_RHO).powi(7) - 1.0);
            let col = COL_BLUE.lerp(COL_RED, rho / REST_RHO);

            self.particles[i].col = col;
            self.particles[i].rho = rho;
            self.particles[i].pressure = pressure;
            self.particles[i].dv = vec2f_zero();
        }
    }

    // Computes the navier stokes update for every particle
    fn update_navierstokes_dv(&mut self, neighbours: &Neighbours, _dt: f32) {
        for i in 0..self.particles.len() {
            let pi = &self.particles[i];
            let neighbours: Vec<&Particle> = neighbours[i]
                                             .iter()
                                             .map(|&j| { &self.particles[j] })
                                             .collect();

            // Compute navier stokes update
            let dv = self.compute_navierstokes_dv(pi, neighbours);
            self.particles[i].dv += dv;
        }
    }

    // Compute the effect of extern forces on particles
    fn update_forces_dv(&mut self, force_neighbours: &Neighbours, _dt: f32) {
        for (force, neighbours) in izip!(self.forces.iter(), force_neighbours.iter()) {
            for j in neighbours.iter() {
                let j = *j;
                let pj = &self.particles[j];
                let dir = (pj.pos - force.pos()).normalize();
                let dist2 = (pj.pos - force.pos()).magnitude2();
                let dv = dir * force.power / dist2;

                self.particles[j].dv += dv;
            }
        }
    }

    // Apply the updated dv to the particle, and ensure that boundaries
    // are satisfied
    fn update_integrate(&mut self, dt: f32) {
        for i in 0..self.particles.len() {
            let pi = &self.particles[i];

            // Find velocity
            let mut vel = pi.vel + pi.dv * dt;

            // Find position
            let pos = pi.pos + vel * dt;

            // Wall bounce update
            vel = self.compute_wall_bounce(&pos, &vel);

            self.particles[i].pos = pos;
            self.particles[i].vel = vel;
        }
    }

    fn compute_navierstokes_dv(&self, pi: &Particle, neighbours: Vec<&Particle>) -> Vector2f {
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
    pub fn debug_single_particle(&mut self) {
        const CHOSEN_IDX: usize = 247;
        let accel = Grid::new(self.width, self.height, H, &self.particles);
        let neighbours = accel.nearest_by_idx(CHOSEN_IDX, H*2.0);
        self.particles[CHOSEN_IDX].col = Color::new(0.0, 0.0, 0.0);
        for j in neighbours.into_iter() {
            self.particles[j].col = Color::new(1.0, 1.0, 0.0);
        }
    }

    pub fn debug_first_force(&mut self) {
        if self.forces.len() == 0 {
            return;
        }
        let accel = Grid::new(self.width, self.height, H, &self.particles);
        let force = &self.forces[0];
        let neighbours = accel.nearest_by_pos(force.pos(), force.r);
        for j in neighbours.into_iter() {
            self.particles[j].col = Color::new(1.0, 1.0, 0.0);
        }
    }

    pub fn debug_check_nans(&self, old_particles: &Vec<Particle>) {
        let mut is_bad = false;
        for (i, pi) in self.particles.iter().enumerate() {
            if !pi.pos.x.is_finite() || !pi.pos.y.is_finite() {
                println!("Found bad particle with idx {}: {:?}", i, pi);

                let accel = Grid::new(self.width, self.height, H, old_particles);
                let neighbours = accel.nearest_by_idx(i, H*2.0);
                println!("Previous frame: {:?}\nNeighbours:{}", old_particles[i], neighbours.len());
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