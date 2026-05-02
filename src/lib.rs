
pub fn calc_hypotenuse(a: f64, b: f64) -> f64 {
    (a.powi(2) + b.powi(2)).sqrt()
}

pub fn calc_other_side_with_hypotenuse(a: f64, hyp: f64) -> f64 {
    (hyp.powi(2) - a.powi(2)).sqrt()
}