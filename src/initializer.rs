use wasm_bindgen::prelude::*;
use rand::Rng;
use image::{GenericImageView};
use crate::Particle;
use crate::util::*;

#[wasm_bindgen]
pub enum Strategy {
    RANDOM,
    DAMBREAK,
    YINYANG,
}

pub fn initialize(strategy: Strategy, width: f32, height: f32, particle_mass: f32) -> Vec<Particle> {
    // Can't use traits because wasm doesn't support it yet
    match strategy {
        Strategy::RANDOM => {
            let mut rng = rand::thread_rng();
            let mut particles = Vec::new();

            for _ in 0..500 {
                let x = rng.gen::<f32>() * width;
                let y = rng.gen::<f32>() * height;

                let position = Vector2f::new(x, y);
                let col = Color::new(0.0, 0.0, 1.0);
                particles.push(Particle{
                    pos: position,
                    col: col,
                    vel: vec2f_zero(),
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
                        mass: particle_mass,
                        rho: 0.0,
                        pressure: 0.0
                    });
                }
            }

            particles
        },
        Strategy::YINYANG => {
            let bytes = include_bytes!("../images/yinyang.png");
            let mut particles = Vec::new();
            let img = image::load_from_memory(bytes).unwrap();
            let mut rng = rand::thread_rng();

            while particles.len() < 500 {
                let rand_x = rng.gen::<f32>();
                let rand_y = rng.gen::<f32>();

                let img_x = (rand_x * img.width()  as f32).floor() as u32;
                let img_y = ((1.0 - rand_y) * img.height() as f32).floor() as u32;
                let pixel = img.get_pixel(img_x, img_y);
                if pixel[3] == 0 {
                    continue;
                }

                let color = Color::new(
                    pixel[0] as f32 / 255.0,
                    pixel[1] as f32 / 255.0,
                    pixel[2] as f32 / 255.0,
                );

                let position = Vector2f::new(
                    rand_x * width,
                    rand_y * height,
                );
                particles.push(Particle{
                    pos: position,
                    col: color,
                    vel: vec2f_zero(),
                    mass: particle_mass,
                    rho: 0.0,
                    pressure: 0.0
                });
            }

            particles
        },
    }
}