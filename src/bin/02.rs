use numrs::{clc::*, E, PI};

fn main() {
    let f = |x: f64| -> f64 { E.powf(x) * x.sin() };
    let x0 = 0.0;
    let x1 = PI;
    let target = (E.powf(PI) + 1.0) / 2.0;
    for i in 1..100 {
        println!(
            "{} | {:.0} {:.0} {:.0}",
            i,
            -(target - integral(&f, x0, x1, i, IntegralMethod::Bar))
                .abs()
                .ln()
                .floor(),
            -(target - integral(&f, x0, x1, i, IntegralMethod::Trapezoid))
                .abs()
                .ln()
                .floor(),
            -(target - integral(&f, x0, x1, i, IntegralMethod::Simpson))
                .abs()
                .ln()
                .floor()
        );
    }
}
