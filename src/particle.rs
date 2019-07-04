use crate::util::{Vector2f, Color};
use crate::accelerators::{HasPosition};

#[repr(C)]
#[derive(Clone)]
pub struct Particle {
    pub pos: Vector2f,
    pub col: Color,
    pub vel: Vector2f,
    pub mass: f32,
    pub rho: f32,
    pub pressure: f32,
}

impl Particle {
    pub fn new(pos: Vector2f, col: Color, vel: Vector2f, mass: f32, rho: f32, pressure: f32) -> Particle {
        Particle{pos, col, vel, mass, rho, pressure}
    }
}

impl HasPosition for Particle {
    fn position(&self) -> Vector2f {
        self.pos
    }
}