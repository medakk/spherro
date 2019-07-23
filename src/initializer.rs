use wasm_bindgen::prelude::*;
use crate::Particle;
use crate::util::*;

#[wasm_bindgen]
pub struct Config {
    width_frac: f32,
    height_frac: f32,
    rows: usize,
    cols: usize,
}

#[wasm_bindgen]
impl Config {
    pub fn new(width_frac: f32, height_frac: f32, rows: usize, cols: usize) -> Config {
        Config {
            width_frac,
            height_frac,
            rows,
            cols,
        }
    }
}

// Creates particles arranged in rows and columns delimited by the fractions
// of width and height provided by the config. Cleaner ways of implementing
// this are difficult because wasm_bindgen doesn't support traits and non-C style
// enums yet
pub fn initialize(config: &Config, width: f32, height: f32, particle_mass: f32) -> Vec<Particle> {
    let mut particles = Vec::new();

    let width  = config.width_frac  * width;
    let height = config.height_frac * height;

    let x_spacing = (width  / config.cols as f32) as usize;
    let y_spacing = (height / config.rows as f32) as usize;

    for i in 0..config.cols {
        for j in 0..config.rows {
            let x = (x_spacing * (i + (j % 2)*3)) as f32;
            let y = (y_spacing * j) as f32;

            let position = Vector2f::new(x, y);
            let color = Color::new(0.0, 0.0, 1.0);
            particles.push(Particle{
                pos: position,
                col: color,
                vel: vec2f_zero(),
                mass: particle_mass,
                rho: 0.0,
                pressure: 0.0
            });
        }
    }

    particles
}