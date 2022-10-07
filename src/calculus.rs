use crate::{EPSILON, F};

pub fn int_bar<FN: Fn(f64) -> f64>(f: &FN, a: F, b: F, m: usize) -> F {
    let mut sum = 0.0;
    let dx = (b - a) / (2.0 * m as f64);
    for k in 0..(2 * m + 1) {
        let kf = k as f64;
        let x = a + kf * dx;
        sum += f(x);
    }
    (sum) * dx
}
pub fn int_trap<FN: Fn(f64) -> f64>(f: &FN, a: F, b: F, m: usize) -> F {
    let mut sum = 0.0;
    let dx = (b - a) / (2.0 * m as f64);
    for k in 1..(2 * m) {
        let kf = k as f64;
        let x = a + kf * dx;
        sum += f(x);
    }
    (sum + (f(a) + f(b)) / 2.0) * dx
}
pub fn int_simpson<FN: Fn(f64) -> f64>(f: &FN, a: F, b: F, m: usize) -> F {
    let mut sum = 0.0;
    let h = (b - a) / (2.0 * m as f64);

    for k in 1..m {
        let kf = k as f64;
        let km1 = a + (2.0 * kf - 1.0) * h;
        let km2 = a + (2.0 * kf) * h;
        sum += 4.0 * f(km1) + 2.0 * f(km2);
    }
    sum += 4.0 * f(a + h * (2 * m - 1) as f64);

    h / 3.0 * (f(a) + sum + f(b))
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
    fn test_barint() {
        let f = |x: f64| x.powi(2);
        println!("{}", int_bar(&f, 0.0, 10.0, 1000));
    }
    #[test]
    fn test_simpson() {
        let f = |x: f64| x.powi(2);
        println!("{}", int_simpson(&f, 0.0, 10.0, 10));
    }
}
