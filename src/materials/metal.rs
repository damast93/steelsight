use crate::*;
use crate::materials::*;

pub struct Metal {
    pub albedo: Color,
    pub fuzz: float // should be in [0,1]
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * (v * n) * n
}

impl Material for Metal {
    fn scatter(&self, ray_in: Ray, hit: &HitRecord, rng: &mut Random) -> Option<Scattering> {
        let reflected = reflect(ray_in.direction.unit(), hit.normal);
        let scattered = reflected + self.fuzz * random::in_unit_sphere(rng);

        if (scattered * hit.normal) > 0.0 {
            Some(Scattering {
                scattered_ray: Ray { origin: hit.p, direction: scattered },
                attenuation: self.albedo
            })
        } else {
            None
        }
    }
}