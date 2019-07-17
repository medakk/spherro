use wasm_bindgen::prelude::*;
use rand::Rng;
use crate::Particle;
use crate::util::*;

#[wasm_bindgen]
pub enum Strategy {
    RANDOM,
    DAMBREAK,
}

pub fn initialize(strategy: Strategy, width: f32, height: f32, particle_mass: f32) -> Vec<Particle> {
    match strategy {
        Strategy::RANDOM => {
            let mut rng = rand::thread_rng();
            let mut particles = Vec::new();

            for _ in 0..500 {
                let x: f32 = rng.gen::<f32>() * width;
                let y: f32 = rng.gen::<f32>() * height;

                let position = Vector2f::new(x, y);
                let col = Color::new(0.0, 0.0, 1.0);
                particles.push(Particle{
                    pos: position,
                    col: col,
                    vel: vec2f_zero(),
                    dv: vec2f_zero(),
                    mass: particle_mass,
                    rho: 0.0,
                    pressure: 0.0
                });
            }

            particles
        },
        Strategy::DAMBREAK => {
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

                    let position = Vector2f::new(x, y);
                    let col = Color::new(0.0, 0.0, 1.0);
                    particles.push(Particle{
                        pos: position,
                        col: col,
                        vel: vec2f_zero(),
                        dv: vec2f_zero(),
                        mass: particle_mass,
                        rho: 0.0,
                        pressure: 0.0
                    });
                }
            }

            particles
        }
    }
}