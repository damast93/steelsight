use crate::*; // Import Vec3, float, etc ... (lib.rs)
use crate::materials::*;

use std::sync::Arc;

// Facade pattern:
// Declare child modules privately 
mod sphere;
mod geometry_list;

// And reexport them immediately
pub use sphere::*;
pub use geometry_list::*;


// Record data of a ray-geometry hit
// The normal is always chosen to be oriented *against* the incident ray
// `front_side` records on which side the surface was hit 
pub struct HitRecord {
       pub t: float,
       pub p: Vec3,
       pub normal: Vec3,
       pub front_side: bool,
       pub material: Arc<dyn Material + Send + Sync> 
}

pub trait Geometry {
       // Returns the nearest intersection in [t_min, t_max] or None
       fn hit(&self, ray: Ray, t_min: float, t_max: float) -> Option<HitRecord>;
}
