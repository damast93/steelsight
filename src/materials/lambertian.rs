use crate::*;
use crate::materials::*;

pub struct Lambertian {
    pub albedo: Color
}

impl Material for Lambertian {
    fn scatter(&self, _ray: Ray, hit: &HitRecord, rng: &mut Random) -> Option<Scattering> {
        let mut scatter_direction = hit.normal + random::unit_vector(rng);

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }

        // Always scatter (TODO: include, sometimes absorption)
        Some(Scattering {
            scattered_ray: Ray { origin: hit.p, direction: scatter_direction },
            attenuation: self.albedo
        })
    }
}