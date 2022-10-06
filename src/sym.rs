struct Variable {}

struct Expression {
    sym: Vec<Variable>,
    exp: &str,
}

impl Expression {
    fn eval() -> f64 {
        0.0
    }
}
