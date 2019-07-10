use crate::util::{Vector2f, Color};
use crate::accelerators::{HasPosition};

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Particle {
    pub pos: Vector2f,
    // The color is more of a way to debug things than an actual property
    // of the particle.
    pub col: Color,
    pub vel: Vector2f,
    pub mass: f32,
    pub rho: f32,
    pub pressure: f32,
}

impl HasPosition for Particle {
    fn position(&self) -> Vector2f {
        self.pos
    }
}