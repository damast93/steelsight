use crate::*;
use crate::materials::*;

pub struct Metal {
    pub albedo: Color
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * (v * n) * n
}

impl Material for Metal {
    fn scatter(&self, ray_in: Ray, hit: &HitRecord, _rng: &mut Random) -> Option<Scattering> {
        let reflected = reflect(ray_in.direction.unit(), hit.normal);

        if (reflected * hit.normal) > 0.0 {
            Some(Scattering {
                scattered_ray: Ray { origin: hit.p, direction: reflected },
                attenuation: self.albedo
            })
        } else {
            None
        }
    }
}