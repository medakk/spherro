extern crate web_sys;

use cgmath::{Vector2, Vector3};

pub type Vector2f = Vector2<f32>;
pub type Color = Vector3<f32>;

pub fn vec2f_zero() -> Vector2f {
    Vector2f::new(0.0, 0.0)
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[inline]
pub fn clamp_f32(v: f32, a: f32, b: f32) -> f32 { b.min(v.max(a)) }