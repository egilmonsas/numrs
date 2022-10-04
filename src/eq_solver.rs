pub fn bisection<F: Fn(f64) -> f64>(
    mut a: f64,
    mut b: f64,
    tol: f64,
    itmax: usize,
    func: F,
) -> f64 {
    let mut x_old = (a + b) / 2.0;
    for it in 0..itmax {
        let mut x = (a + b) / 2.0;

        if (x - x_old).abs() < tol && it > 0 {
            return x;
        }

        let y_left = func(a);
        let y_mid = func(x);
        let y_right = func(b);
        println!(" {:03} | f({:.4}) = {:.4}", it, x, y_mid);

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
    0.0
}
