extern crate web_sys;

use noisy_float::prelude::*;

use cgmath::{Vector2, Vector3};
use cgmath::BaseFloat;

pub type Vector2f = Vector2<R32>;
pub type Color = Vector3<R32>;

impl BaseFloat for R32 {}

pub fn vec2f_zero() -> Vector2f {
    Vector2f::new(r32(0.0), r32(0.0))
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
pub fn clamp_R32(v: R32, a: R32, b: R32) -> R32 { b.min(v.max(a)) }