use wasm_bindgen::prelude::*;

//TODO:
// * Operator overloading

#[wasm_bindgen]
#[derive(Clone)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

#[wasm_bindgen]
impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3{x, y, z}
    }

    pub fn add(&self, other: &Vector3) -> Vector3 {
        Vector3::new(self.x+other.x, self.y+other.y, self.z+other.z)
    }
}