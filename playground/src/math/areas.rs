pub fn area_of_circle(radius: f64) -> f64 {
    std::f64::consts::PI * radius.powi(2)
}

pub fn area_of_square(side: u32) -> u32 {
    side.pow(2)
}
