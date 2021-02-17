use crate::*;
use crate::geometry::*;

mod lambertian;
mod metal;
mod dielectric;

pub use lambertian::*;
pub use metal::*;
pub use dielectric::*;

pub struct Scattering {
    pub scattered_ray: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, ray: Ray, hit: &HitRecord, rng: &mut Random) -> Option<Scattering>;
}