use rand::Rng;
use crate::Particle;
use crate::util::*;

pub enum Strategy {
    RANDOM,
    DAMBREAK,
}

pub fn initialize(width: f32, height: f32, strategy: Strategy) -> Vec<Particle> {
    match strategy {
        Strategy::RANDOM => {
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

                    let position = Vector3f::new(x, y, 0.0);
                    let col = Vector3f::new(0.0, 0.0, 1.0);
                    particles.push(Particle::new(position, col, vec3f_zero(), 100.0, 1.0, 1.0));
                }
            }

            particles
        }
    }
}