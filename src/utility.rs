#[allow(non_camel_case_types)]
pub type float = f64;

pub use std::f64::consts::{PI};
pub use rand::prelude::*;

// Fix a type of Rng here, the polymorphim messes up trait objects with stochastic methods
pub type Random = ThreadRng;

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
    
    pub fn vector(min: float, max: float, rng: &mut Random) -> Vec3 {
        vec3(rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max))
    }

    pub fn in_unit_sphere(rng: &mut Random) -> Vec3 {
        let mut v = random::vector(-1.0, 1.0, rng);
        while v.length_squared() >= 1.0 {
            v = random::vector(-1.0, 1.0, rng);
        }
        v
    }

    pub fn unit_vector(rng: &mut Random) -> Vec3 {
        random::in_unit_sphere(rng).unit()
    }

    pub fn in_unit_disk(rng: &mut Random) -> Vec3 {
        let mut v = vec3(rng.gen_range(-1.0..1.0),rng.gen_range(-1.0..1.0), 0.0);
        while v.length_squared() >= 1.0 {
            v = vec3(rng.gen_range(-1.0..1.0),rng.gen_range(-1.0..1.0), 0.0)
        }
        v
    }
}