

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

pub mod random {
    use crate::*;
    use rand::prelude::*;
    
    pub fn vector(rng: &mut impl Rng, min: float, max: float) -> Vec3 {
        vec3(rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max))
    }

    pub fn in_unit_sphere(rng: &mut impl Rng) -> Vec3 {
        let mut v = random::vector(rng, -1.0, 1.0);
        while v.length_squared() > 1.0 {
            v = random::vector(rng, -1.0, 1.0);
        }
        v
    }

    pub fn unit_vector(rng: &mut impl Rng) -> Vec3 {
        random::in_unit_sphere(rng).unit()
    }
}