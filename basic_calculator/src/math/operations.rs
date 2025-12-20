pub fn add(lhs: f64, rhs: f64) -> f64 {
    lhs + rhs
}

pub fn subtract(lhs: f64, rhs: f64) -> f64 {
    lhs - rhs
}

pub fn multiply(lhs: f64, rhs: f64) -> f64 {
    lhs * rhs
}

pub fn divide(lhs: f64, rhs: f64) -> Option<f64> {
    if rhs == 0.0 {
        return None;
    }
    Some(lhs / rhs)
}
