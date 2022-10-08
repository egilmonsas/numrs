pub mod calculus;
pub mod eq_solve;
pub mod ode;
pub mod utils;

pub struct XY_point {
    x: F,
    y: F,
}

type F = f64;
pub const EPSILON: F = 0.0000001;
pub const E: F = std::f64::consts::E;
pub const PI: F = std::f64::consts::PI;
