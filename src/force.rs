use wasm_bindgen::prelude::*;
use crate::util::*;

use noisy_float::prelude::*;

pub struct Force {
    //TODO: using x,y instead of Vector2f because of lack of
    //wasm_bindgen in Vector2f
    pub x: R32,
    pub y: R32,
    pub power: R32,
    pub r: R32,
}

impl Force {
    pub fn new(x: R32, y: R32, power: R32, r: R32) -> Force {
        Force{
            x: x,
            y: y,
            power: power,
            r: r,
        }
    }
}

impl Force {
    pub fn pos(&self) -> Vector2f {
        Vector2f::new(self.x, self.y)
    }
}