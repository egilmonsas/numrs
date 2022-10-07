use numrs::{calculus::*, E, PI};

fn main() {
    let f = |x: f64| -> f64 { E.powf(x) * x.sin() };
    let x0 = 0.0;
    let x1 = PI;
    let target = (E.powf(PI) + 1.0) / 2.0;
    for i in 1..100 {
        println!(
            "{} | {:.0} {:.0} {:.0}",
            i,
            -(target - integral(&f, x0, x1, i, 1)).abs().ln().floor(),
            -(target - integral(&f, x0, x1, i, 2)).abs().ln().floor(),
            -(target - integral(&f, x0, x1, i, 3)).abs().ln().floor()
        );
    }
}
