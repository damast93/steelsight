#[allow(non_camel_case_types)]
pub type float = f64;

pub use std::f64::consts::{PI};

pub fn deg_to_rad(deg: float) -> float {
    deg * PI / 180.0
}

pub fn clamp(x: float, min: float, max: float) -> float {
    if x < min { min }
    else if x > max { max }
    else { x }
}