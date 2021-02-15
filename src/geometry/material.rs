use crate::*;
use crate::geometry::*;

pub struct Scattering {
    pub scattered_ray: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, ray: Ray, hit: &HitRecord, rng: &mut Random) -> Option<Scattering>;
}