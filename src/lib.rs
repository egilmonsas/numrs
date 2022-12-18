pub mod clc;
pub mod eqs;
pub mod mth;
pub mod ode;
pub mod rsarray;
pub mod utl;

use std::fmt;

#[derive(Clone, Copy)]
pub struct Vec2 {
    x: F,
    y: F,
}
impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {:.2} y: {:.2}", self.x, self.y)
    }
}
type F = f64;
pub const EPSILON: F = 0.0000001;
pub const E: F = std::f64::consts::E;
pub const PI: F = std::f64::consts::PI;
