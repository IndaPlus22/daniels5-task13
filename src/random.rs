use rand::Rng;

pub fn random_double(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    return min + (max-min)*rng.gen::<f64>();
}

pub fn clamp(x: f64, min: f64, max: f64)-> f64 {
    if x < min {return min}
    if x > max {return max}
    x
}