use crate::*;
use crate::materials::*;

pub struct Dielectric {
    pub eta: float // index of refraction
}

// Something still off? I don't get the same picture as in the tutorial?

fn refract(uv : Vec3, n : Vec3, eta_ratio : float) -> Vec3 {
    let cos_theta = (-(uv * n)).min(1.0);
    let r_out_perp = eta_ratio * (uv + cos_theta * n);
    let r_out_par = (-1.0) * (1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
    r_out_perp + r_out_par
}

fn reflectance(cosine: float, ref_idx: float) -> float {
    // Schlick's approximation for reflectance
    let r0 = (1.0-ref_idx) / (1.0+ref_idx);
    let r = r0*r0;
    r + (1.0-r)*(1.0-cosine).powi(5)
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: Ray, hit: &HitRecord, rng: &mut Random) -> Option<Scattering> {
        let attenuation = Color::from_rgb(1.0, 1.0, 1.0);

        let eta_ratio = if hit.front_side {1.0/self.eta} else {self.eta};
        let unit_dir = ray_in.direction.unit();
        let cos_theta = (-(unit_dir * hit.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
        let cannot_refract = eta_ratio * sin_theta > 1.0;

        // Randomly choose between reflecting and refracting (if possible)
        let do_reflection =
          cannot_refract
          || (reflectance(cos_theta, eta_ratio) > rng.gen());

        let direction = 
          if do_reflection 
                 { reflect(unit_dir, hit.normal) } 
            else { refract(unit_dir, hit.normal, eta_ratio) };
        
        let scattered_ray = Ray { origin: hit.p, direction };

        Some(Scattering { attenuation, scattered_ray })
    }
}