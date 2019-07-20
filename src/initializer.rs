use wasm_bindgen::prelude::*;
use rand::Rng;
use noisy_float::prelude::*;
use num_traits::cast::ToPrimitive;
use crate::Particle;
use crate::util::*;

#[wasm_bindgen]
pub enum Strategy {
    DAMBREAK,
}

pub fn initialize(strategy: Strategy, width: R32, height: R32, particle_mass: R32) -> Vec<Particle> {
    match strategy {
        Strategy::DAMBREAK => {
            let mut particles = Vec::new();

            let width = r32(0.40) * width;
            let height = r32(0.80) * height;

            let rows = r32(50.0);
            let cols = r32(10.0);

            let x_spacing = (width / cols).to_usize().unwrap();
            let y_spacing = (height / rows).to_usize().unwrap();

            for i in 0..cols.to_usize().unwrap() {
                for j in 0..rows.to_usize().unwrap() {
                    let x = r32((x_spacing * (i + (j % 2)*3)) as f32);
                    let y = r32((y_spacing * j) as f32);

                    let position = Vector2f::new(x, y);
                    let col = Color::new(r32(0.0), r32(0.0), r32(0.0));
                    particles.push(Particle{
                        pos: position,
                        col: col,
                        vel: vec2f_zero(),
                        mass: particle_mass,
                        rho: r32(0.0),
                        pressure: r32(0.0),
                    });
                }
            }

            particles
        }
    }
}