pub trait Kernel {
    fn f(&self, q: f32) -> f32;
    fn df(&self, q: f32) -> f32;
}

pub struct CubicSpline;
impl Kernel for CubicSpline {
    fn f(&self, q: f32) -> f32 {
        if 0.0 <= q && q < 1.0 {
            (2.0/3.0) - (q*q) + (0.5*q*q*q)
        } else if 1.0 <= q && q < 2.0 {
            (1.0/6.0) * (2.0 - q).powi(3)
        } else {
            0.0
        }
    }

    fn df(&self, q: f32) -> f32 {
        if 0.0 <= q && q < 1.0 {
            (-2.0 * q) + (1.5 * q*q)
        } else if 1.0 <= q && q < 2.0 {
            -0.5 * (2.0 - q).powi(2)
        } else {
            0.0
        }
    }
}