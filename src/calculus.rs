use crate::{EPSILON, F};

pub fn integral<FN: Fn(f64) -> f64>(f: &FN, a: F, b: F) -> F {
    let mut x = a;
    let dx = (b - a) / 10000.0;
    let mut sum = 0.0;
    while x < b {
        x += dx;
        sum += f(x) * dx;
    }
    sum
}
pub fn slope_at<FN: Fn(f64) -> f64>(f: &FN, x: f64) -> f64 {
    let h = EPSILON;
    (f(x + h) - f(x)) / h
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_zeq, utils::ZEq};

    #[test]
    fn test_derivative() {
        let f = |x: f64| x.powi(2);
        assert_zeq!(slope_at(&f, 2.0), 4.0)
    }
    #[test]
    fn test_integral() {
        let f = |x: f64| x.powi(2);
        println!("{}", integral(&f, 0.0, 10.0));
    }
}
