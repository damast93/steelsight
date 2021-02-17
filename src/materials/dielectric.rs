use crate::*;
use crate::materials::*;

pub struct Dielectric {
    pub eta: float // index of refraction
}

fn refract(uv : Vec3, n : Vec3, eta_ratio : float) -> Vec3 {
    let cos_theta = (-(uv * n)).min(1.0);
    let r_out_perp = eta_ratio * (uv + cos_theta * n);
    let r_out_par = n * -((1.0 - r_out_perp.length_squared()).abs().sqrt());
    r_out_perp + r_out_par
}

// First: A material that always refracts
impl Material for Dielectric {
    fn scatter(&self, ray_in: Ray, hit: &HitRecord, _rng: &mut Random) -> Option<Scattering> {
        let attenuation = Color::from_rgb(1.0, 1.0, 1.0);
        let eta_ratio = if hit.front_side {1.0/self.eta} else {self.eta};
        let unit_dir = ray_in.direction.unit();
        let refracted = refract(unit_dir, hit.normal, eta_ratio);
        let scattered_ray = Ray { origin: hit.p, direction: refracted };

        Some(Scattering { attenuation, scattered_ray })
    }
}