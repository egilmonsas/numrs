use crate::{Vec2, F};

pub enum OdeMethod {
    Rk4,
    Naive,
}

pub fn compute_next(method: OdeMethod, dy: &dyn Fn(&Vec2) -> f64, p0: Vec2, h: F) -> Vec2 {
    match method {
        OdeMethod::Rk4 => rk4(dy, p0, h),
        OdeMethod::Naive => naive(dy, p0, h),
    }
}

fn naive(dy: &dyn Fn(&Vec2) -> f64, p0: Vec2, h: F) -> Vec2 {
    Vec2 {
        x: p0.x + h,
        y: p0.y + h * dy(&p0),
    }
}

fn rk4(dy: &dyn Fn(&Vec2) -> f64, p0: Vec2, h: F) -> Vec2 {
    let k1 = dy(&p0);
    let k2 = dy(&Vec2 {
        x: p0.x + h / 2.0,
        y: p0.y + h * k1 / 2.,
    });
    let k3 = dy(&Vec2 {
        x: p0.x + h / 2.0,
        y: p0.y + h * k2 / 2.0,
    });
    let k4 = dy(&Vec2 {
        x: p0.x + h,
        y: p0.y + h * k3,
    });
    Vec2 {
        x: p0.x + h,
        y: p0.y + (k1 + (k2 + k3) * 2.0 + k4) / 6.0 * h,
    }
}
