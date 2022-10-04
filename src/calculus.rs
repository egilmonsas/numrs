pub fn slope_at<F: Fn(f64) -> f64>(f: &F, x: f64) -> f64 {
    let h = crate::EPSILON;
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
}
