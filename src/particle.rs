use crate::util::{Vector3f};
use crate::accelerators::{HasPosition};

#[repr(C)]
#[derive(Clone)]
pub struct Particle {
    pub pos: Vector3f,
    pub col: Vector3f,
    pub vel: Vector3f,
    pub mass: f32,
    pub rho: f32,
    pub pressure: f32,
}

impl Particle {
    pub fn new(pos: Vector3f, col: Vector3f, vel: Vector3f, mass: f32, rho: f32, pressure: f32) -> Particle {
        Particle{pos, col, vel, mass, rho, pressure}
    }
}

impl HasPosition for Particle {
    fn position(&self) -> Vector3f {
        self.pos
    }
}