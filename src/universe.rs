use wasm_bindgen::prelude::*;
use cgmath::{InnerSpace};
use rand::Rng;
use crate::util::*;
use crate::particle::{Particle};
use crate::accelerators::{Accelerator, Grid};
use crate::initializer;
use crate::kernel::*;
use crate::force::Force;

const MASS: f32 = 100.0;
const H: f32 = 35.0;
const VISC: f32 = 0.5;
const REST_RHO: f32 = MASS / (130.0 * 130.0);

// This is hack to kill extremely large interaction forces
// that cause the simulation to explode. We cap the force 
// to this value. Note that this is scaled by dt
const MAX_FORCE_MAG: f32 = 450.0;

// Boundary parameters
const BOUNDARY_COR: f32 = 0.5; // Coefficient of restitution
const BOUNDARY_MIN_DV: f32 = 500.0; // If particle is too slow, it is accelerated to atleast this much

const GRAVITY: f32 = -10000.0;
const K: f32 = 10.0;

enum Event {
    Spawn(usize, Vector2f),
    Despawn(usize),
}

#[wasm_bindgen]
pub struct Universe {
    particles: Vec<Particle>,
    width: f32,
    height: f32,
    forces: Vec<Force>,
    events: Vec<Event>,
}

type Neighbours = Vec<Vec<usize>>;

#[wasm_bindgen]
#[allow(non_snake_case)]
impl Universe {
    pub fn new(width: f32, height: f32, config: &initializer::Config) -> Universe {
        if cfg!(target_arch="wasm32") {
            set_panic_hook();
        }

        let particles = initializer::initialize(config, width, height, MASS);

        Universe {
            particles: particles,
            width: width,
            height: height,
            forces: Vec::new(),
            events: Vec::new(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        // This assumes that the neighbours remain the same for the
        // entire update
        let (neighbours, force_neighbours) = self.compute_neighbours();

        self.update_particle_fields(&neighbours);
        self.update_nonpressure_forces(&neighbours, &force_neighbours, dt);

        for _ in 0..4 { //TODO: this condition should take density error
            self.update_particle_fields(&neighbours);
            self.update_pressure_forces(&neighbours, dt);
        }

        self.update_boundary();

        self.update_events();
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

    pub fn queue_spawn_particles(&mut self, count: usize, x: f32, y: f32) {
        let pos = Vector2f::new(x, y);
        self.events.push(Event::Spawn(count, pos));
    }

    pub fn queue_despawn_particles(&mut self, count: usize) {
        self.events.push(Event::Despawn(count));
    }

    pub fn is_unstable(&self) -> bool {
        self.particles.iter().any(|pi| {
            !pi.pos.x.is_finite() || !pi.pos.y.is_finite()
        })
    }
}

#[allow(non_snake_case)]
impl Universe {

    pub fn get_particles(&self) -> &Vec<Particle> {
        &self.particles
    }

    // Updates the density and pressure for every particle
    fn update_particle_fields(&mut self, neighbours: &Neighbours) {
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
            self.particles[i].rho = rho;
            self.particles[i].pressure = pressure;
        }
    }

    // Performs the first part of the splitting solver: updates position and velocity
    // without considering forces which arise from differences in pressure
    fn update_nonpressure_forces(&mut self, neighbours: &Neighbours, force_neighbours: &Neighbours, dt: f32) {
        let mut force_dv = vec![vec2f_zero(); self.particles.len()];

        // Forces update
        for (force, neighbours) in izip!(self.forces.iter(), force_neighbours.iter()) {
            for j in neighbours.iter() {
                let pj = &self.particles[*j];

                let dir = (pj.pos - force.pos()).normalize();
                let dist2 = (pj.pos - force.pos()).magnitude2();
                let mag = (force.power / dist2).min(MAX_FORCE_MAG / dt);
                let vel = dir * mag;

                force_dv[*j] += vel;
            }
        }

        // Viscosity and gravity update
        let gravity_dv = Vector2f::new(0.0, GRAVITY);
        for i in 0..self.particles.len() {
            let pi = &self.particles[i];
            let neighbours: Vec<&Particle> = neighbours[i]
                                            .iter()
                                            .map(|&j| { &self.particles[j] })
                                            .collect();

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

            // Compute viscosity
            let ddv = 2.0 * izip!(&neighbours, &x_ijs, &dWs).map(|(pj, x_ij, dW)| {
                let q1 = (pj.mass / pj.rho) * (pi.vel - pj.vel);
                let q2 = (x_ij.dot(*dW)) / (x_ij.dot(*x_ij) + 0.01*H*H);
                q1 * q2
            }).sum::<Vector2f>();

            let vel = pi.vel
                    + (VISC * ddv + gravity_dv + force_dv[i]) * dt;

            self.particles[i].vel = vel;
            self.particles[i].pos += vel * dt;
        }
    }

    // Performs the second part of the splitting solver: updates position and velocity
    // with only pressure forces
    fn update_pressure_forces(&mut self, neighbours: &Neighbours, dt: f32) {
        for i in 0..self.particles.len() {
            let pi = &self.particles[i];
            let neighbours: Vec<&Particle> = neighbours[i]
                                            .iter()
                                            .map(|&j| { &self.particles[j] })
                                            .collect();

            // Compute gradient of W
            let dWs: Vec<Vector2f> = neighbours.iter().map(|pj| {
                let x_ij = pi.pos - pj.pos;
                let q = x_ij.magnitude() / H;
                let df = cubicspline_df(q);

                let dq = (pi.pos - pj.pos) / (H * q); // gradient of q
                let dW = (1.0 / H.powi(3)) * df * dq;

                dW
            }).collect();

            let dP = pi.rho * izip!(&neighbours, &dWs).map(|(pj, dW)| {
                pj.mass * (pi.pressure / pi.rho.powi(2) + pj.pressure / pj.rho.powi(2)) * dW
            }).sum::<Vector2f>();

            let p_dv = -dP / pi.rho;

            self.particles[i].vel += dt * p_dv;
            self.particles[i].pos += dt * dt * p_dv;
        }
    }

    // the first return value is the neighbours for each particle,
    // the second return value is the neighbours for all the forces
    fn compute_neighbours(&self) -> (Neighbours, Neighbours) {
        let accel = Grid::new(self.width, self.height, H, &self.particles);
        let neighbours: Neighbours = (0..self.particles.len()).map(|i| {
            accel.nearest_by_idx(i, H*2.0)
        }).collect();
        let force_neighbours: Neighbours = self.forces.iter().map(|f| {
            accel.nearest_by_pos(f.pos(), f.r)
        }).collect();

        (neighbours, force_neighbours)
    }

    // Reverses velocity if the particle is outside bounds
    fn update_boundary(&mut self) {
        for i in 0..self.particles.len() {
            let pi = &self.particles[i];
            self.particles[i].vel = self.boundary_correction_vel(&pi.pos, &pi.vel);
        }
    }

    fn boundary_correction_vel(&self, pos: &Vector2f, vel: &Vector2f) -> Vector2f {
        //TODO: This doesn't consider dt

        let mut vel: Vector2f = *vel;

        // Bounce off walls
        if pos.x < 0.0 {
            vel.x = ( BOUNDARY_MIN_DV).max(-BOUNDARY_COR*vel.x);
        } else if pos.x > self.width {
            vel.x = (-BOUNDARY_MIN_DV).min(-BOUNDARY_COR*vel.x);
        } else if pos.y < 0.0 {
            vel.y = ( BOUNDARY_MIN_DV).max(-BOUNDARY_COR*vel.y);
        } else if pos.y > self.height {
            vel.y = (-BOUNDARY_MIN_DV).min(-BOUNDARY_COR*vel.y);
        }

        vel
    }

    // Handles the particle spawning and despawning events
    pub fn update_events(&mut self) {
        let mut rng = rand::thread_rng();

        for event in self.events.iter() {
            match event {
                Event::Spawn(count, pos) => {
                    for _ in 0..*count {
                        // If we cluster all the points at the exact same location,
                        // the pressure force will become extremly high and destabilize the
                        // simulation
                        let x: f32 = pos.x + (rng.gen::<f32>() - 0.5) * (0.3 * H);
                        let y: f32 = pos.y + (rng.gen::<f32>() - 0.5) * (0.3 * H);
                        let pi = Particle{
                            pos: Vector2f::new(x, y),
                            vel: vec2f_zero(),
                            col: Color::new(0.0, 0.0, 1.0),
                            mass: MASS,
                            rho: 0.0,
                            pressure: 0.0,
                        };
                        self.particles.push(pi);
                    }
                },
                Event::Despawn(count) => {
                    // We remove the particles which have the highest velocity
                    // This is a heuristic to prevent destabilization
                    assert!(*count <= self.particles.len());

                    let mut s: Vec<(f32, usize)> = self.particles
                                                  .iter()
                                                  .enumerate()
                                                  .map(|(idx, pi)| (pi.vel.magnitude2(), idx))
                                                  .collect();

                    // TODO: Use a faster sort algorithm, we only need top-k
                    s.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
                    let order: Vec<_> = s.iter().map(|t| t.1).collect();
                    let top_k = &order[order.len()-count-1 .. order.len()];

                    self.particles = self.particles
                                    .clone() //TODO: Avoid this clone
                                    .into_iter()
                                    .enumerate()
                                    .filter(|(idx, _)| !top_k.contains(idx))
                                    .map(|t| t.1)
                                    .collect();
                }
            }
        }
        self.events.clear();
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