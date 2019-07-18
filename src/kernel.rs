// TODO: The formula here is the same as in the SPH Fluids in Computer Graphics
// paper, but according to https://pysph.readthedocs.io/en/latest/reference/kernels.html
// there are different constant factors depending on the dimensionality

#[inline]
pub fn cubicspline_f(q: f32) -> f32 {
    if 0.0 <= q && q < 1.0 {
        (2.0 / 3.0) + q.powi(2) * (q / 2.0 - 1.0)
    } else if 1.0 <= q && q < 2.0 {
        (1.0 / 6.0) * (2.0 - q).powi(3)
    } else {
        0.0
    }
}

#[inline]
pub fn cubicspline_df(q: f32) -> f32 {
    if 0.0 <= q && q < 1.0 {
        q * (-2.0 + 1.5 * q)
    } else if 1.0 <= q && q < 2.0 {
        -((2.0 - q).powi(2)) / 2.0
    } else {
        0.0
    }
}