use crate::util::{Vector2f, Color};
use crate::accelerators::{HasPosition};

use noisy_float::prelude::*;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Particle {
    pub pos: Vector2f,
    pub vel: Vector2f,
    pub mass: R32,
    pub rho: R32,
    pub pressure: R32,

    // The color is more of a way to debug things than an actual property
    // of the particle.
    pub col: Color,
}

impl HasPosition for Particle {
    fn position(&self) -> Vector2f {
        self.pos
    }
}