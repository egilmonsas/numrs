use crate::{clc::slope_at, utl::ZEq};
/// Solution of f(x) = 0, by means on the bisection method
///
/// # Example
/// ```
/// use numrs::assert_zeq;
/// use crate::numrs::utils::ZEq;
///
/// let func = |x: f64| -> f64 { x.powi(3) - x - 4.0 };
/// let sol = numrs::eq_solve::bisection(1.0, 2.0, func);
/// assert_zeq!(sol, 1.796322)
/// ```
pub fn bisection<F: Fn(f64) -> f64>(mut a: f64, mut b: f64, func: F) -> f64 {
    let mut x_old = (a + b) / 2.0;
    for it in 0..100 {
        let mut x = (a + b) / 2.0;

        if x.zeq(x_old) && it > 0 {
            return x;
        }

        let y_left = func(a);
        let y_mid = func(x);
        let y_right = func(b);
        println!(" {:03} | f({:.3}) = {:.3}", it, x, y_mid);

        if y_left * y_right > 0.0 {
            panic!("Function does not cross 0 {:.2}..{:.2}", y_left, y_right)
        }
        if y_left * y_mid < 0.0 {
            a = a;
            b = x;
        } else {
            a = x;
            b = b;
        }
        x_old = std::mem::take(&mut x);
    }
    panic!("Could not find a solution")
}

pub fn newtons_method<F: Fn(f64) -> f64>(mut x: f64, f: F) -> f64 {
    let mut x_old = x;
    for it in 0..100 {
        let y = f(x);
        let dy = slope_at(&f, x);

        println!(" {:03} | f({:.3}) = {:.3}", it, x, y);
        if x.zeq(x_old) && it > 0 {
            return x;
        }
        x_old = std::mem::take(&mut x);
        x = x_old - y / dy;
    }
    panic!("Could not find a solution")
}

#[cfg(test)]
mod test {
    use crate::{assert_zeq, utl::ZEq};

    use super::*;
    #[test]
    fn test_bisection() {
        let func = |x: f64| -> f64 { x.powi(3) - x - 4.0 };

        let sol = bisection(1.0, 2.0, func);
        assert_zeq!(sol, 1.796322)
    }
    #[test]
    fn test_newton() {
        let func = |x: f64| -> f64 { x.powi(3) - x - 4.0 };

        let sol = newtons_method(1.9, func);
        assert_zeq!(sol, 1.796322)
    }
}
