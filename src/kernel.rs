use std::f32::consts::PI;

// https://pysph.readthedocs.io/en/latest/reference/kernels.html
const _SIGMA_1D: f32 = 2.0 / 3.0;
const  SIGMA_2D: f32 = 10.0 / (7.0 * PI);
const _SIGMA_3D: f32 = 1.0 / PI;

const SIGMA: f32 = SIGMA_2D;

#[inline]
pub fn cubicspline_f(q: f32) -> f32 {
    if 0.0 <= q && q < 1.0 {
        SIGMA * (1.0 - 1.5 * q.powi(2) * (1.0 - q / 2.0))
    } else if 1.0 <= q && q < 2.0 {
        (SIGMA / 4.0) * (2.0 - q).powi(3)
    } else {
        0.0
    }
}

#[inline]
pub fn cubicspline_df(q: f32) -> f32 {
    if 0.0 <= q && q < 1.0 {
        3.0 * SIGMA * q * (0.75 * q - 1.0)
    } else if 1.0 <= q && q < 2.0 {
        -0.75 * SIGMA * (2.0 - q).powi(2)
    } else {
        0.0
    }
}