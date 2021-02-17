use crate::geometry::*;
use std::sync::Arc;

// Design decision: Now objects own/share their materials
pub struct Sphere {
    pub center: Vec3,
    pub radius: float,
    pub material: Arc<dyn Material + Send + Sync>
}

impl Geometry for Sphere {
    fn hit(&self, r: Ray, t_min: float, t_max: float) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc * r.direction;
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        // No intersection
        if discriminant < 0.0 { return None }

        let sqrtd = discriminant.sqrt();

        // Try smaller root first
        let mut root = (-half_b - sqrtd) / a;
        if (root < t_min) || (t_max < root) {
            // Else try larger root
            root = (-half_b + sqrtd) / a;
            if (root < t_min) || (t_max < root) {
                return None;
            } 
        }

        let p = r.at(root);
        let out_normal = (p - self.center) / self.radius;
        let (normal,front_side) = Ray::orient_normal(r, out_normal);
        Some(HitRecord { t: root, p, normal, front_side, material: self.material.clone() })
    }
}