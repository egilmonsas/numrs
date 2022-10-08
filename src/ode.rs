use crate::{Vec2, F};

pub enum OdeMethod {
    Rk4,
    Naive,
}

pub fn step_forward<FN: Fn(Vec2) -> f64>(method: OdeMethod, dy: &FN, p0: Vec2, h: F) -> Vec2 {
    match method {
        OdeMethod::Rk4 => rk4(dy, p0, h),
        OdeMethod::Naive => naive(dy, p0, h),
    }
}

fn naive<FN: Fn(Vec2) -> f64>(dy: &FN, p0: Vec2, h: F) -> Vec2 {
    Vec2 {
        x: p0.x + h,
        y: p0.y + h * dy(p0),
    }
}

fn rk4<FN: Fn(Vec2) -> f64>(dy: &FN, p0: Vec2, h: F) -> Vec2 {
    let k1 = dy(p0);
    let k2 = dy(Vec2 {
        x: p0.x + h / 2.0,
        y: p0.y + h * k1 / 2.,
    });
    let k3 = dy(Vec2 {
        x: p0.x + h / 2.0,
        y: p0.y + h * k2 / 2.0,
    });
    let k4 = dy(Vec2 {
        x: p0.x + h,
        y: p0.y + h * k3,
    });
    Vec2 {
        x: p0.x + h,
        y: p0.y + (k1 + (k2 + k3) * 2.0 + k4) / 6.0 * h,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rk4() {
        let p0 = Vec2 { x: 0.0, y: 1.0 };
        let h = 0.5;
        let dy = |p: Vec2| p.x.sin().powi(2) * p.y;
        let res = step_forward(OdeMethod::Rk4, &dy, p0, h);
        println!("{}", res)
    }
}
