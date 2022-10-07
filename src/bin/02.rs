use numrs::calculus::*;

fn main() {
    let f = |x: f64| -> f64 { x.powi(3) - x.powi(2) + x.ln() - 2.0 };
    let target = 2162.7758509299404568401799145468436420760110148862877297603332790;
    for i in 1..100 {
        println!(
            "{} | {:.0} {:.0} {:.0}",
            i,
            -(target - int_bar(&f, 1.0, 10.0, i)).abs().ln().floor(),
            -(target - int_trap(&f, 1.0, 10.0, i)).abs().ln().floor(),
            -(target - int_simpson(&f, 1.0, 10.0, i)).abs().ln().floor()
        );
    }
}
