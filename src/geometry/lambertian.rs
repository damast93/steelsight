use crate::*;
use crate::geometry::*;

// Put into `materials` module
pub struct Lambertian {
    pub albedo: Color
}

impl Material for Lambertian {
   
    fn scatter(&self, _ray: Ray, hit: &HitRecord, rng: &mut Random) -> Option<Scattering> {
        let scatter_direction = hit.normal + random::unit_vector(rng);

        Some(Scattering {
            scattered_ray: Ray { origin: hit.p, direction: scatter_direction },
            attenuation: self.albedo
        })
    }
}