use crate::*; // Import Vec3, float, etc ... (lib.rs)

// Facade pattern:
// Declare child modules privately 
mod material;
mod sphere;
mod geometry_list;

pub mod lambertian;

// And reexport them immediately
pub use material::*;
pub use sphere::*;
pub use geometry_list::*;


// Record data of a ray-geometry hit
// The normal is always chosen to be oriented *against* the incident ray
// `front_side` records on which side the surface was hit 
pub struct HitRecord<'scene> {
       pub t: float,
       pub p: Vec3,
       pub normal: Vec3,
       pub front_side : bool,
       pub material : &'scene dyn Material 
}

pub trait Geometry {
       // Returns the nearest intersection in [t_min, t_max] or None
       fn hit(&self, ray: Ray, t_min: float, t_max: float) -> Option<HitRecord>;
}
