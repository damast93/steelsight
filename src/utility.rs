#[allow(non_camel_case_types)]
pub type float = f64;

pub use std::f64::consts::{PI};

fn deg_to_rad(deg : float) -> float {
    deg * PI / 180.0
}