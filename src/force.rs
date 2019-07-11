use wasm_bindgen::prelude::*;
use crate::util::*;

#[wasm_bindgen]
pub struct Force {
    //TODO: using x,y instead of Vector2f because of lack of
    //wasm_bindgen in Vector2f
    pub x: f32,
    pub y: f32,
    pub power: f32,
    pub r: f32,
}

#[wasm_bindgen]
impl Force {
    pub fn new(x: f32, y: f32, power: f32, r: f32) -> Force {
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