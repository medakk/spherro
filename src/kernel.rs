pub trait Kernel {
    fn f(&self, q: f32) -> f32;
    fn df(&self, q: f32) -> f32;
}

pub struct CubicSpline;
impl Kernel for CubicSpline {
    fn f(&self, q: f32) -> f32 {
        if 0.0 <= q && q < 1.0 {
            (2.0 / 3.0) - q.powi(2) * (1.0 + q / 2.0)
        } else if 1.0 <= q && q < 2.0 {
            (1.0 / 6.0) * (2.0 - q).powi(3)
        } else {
            0.0
        }
    }

    fn df(&self, q: f32) -> f32 {
        if 0.0 <= q && q < 1.0 {
            q * (-2.0 + 1.5 * q)
        } else if 1.0 <= q && q < 2.0 {
            -(2.0 - q).powi(2) / 2.0
        } else {
            0.0
        }
    }
}