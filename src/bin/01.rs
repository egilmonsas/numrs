use numrs::eqs::bisection;

fn main() {
    let a = 1.0;
    let b = 2.0;
    let func = |x: f64| -> f64 { x.powi(3) - x - 4.0 };

    let sol = bisection(a, b, func);

    println!("Found solution at x = {:.5}", sol)
}
