use crate::*; // Import Vec3, float, etc ... (lib.rs)

// Facade pattern
mod sphere; // Declare the sphere module (sphere.rs) privately 
pub use sphere::*; // Immediately reexport it

pub struct HitRecord {
       pub t: float,
       pub p: Vec3,
       pub normal: Vec3,
       pub front_side : bool
}

pub trait Geometry {
       // Returns the nearest intersection in [t_min, t_max] or None
       fn hit(&self, r: Ray, t_min: float, t_max: float) -> Option<HitRecord>;
}