use super::vec3::*;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
       pub origin : Vec3,
       pub direction : Vec3
}

impl Ray {
       pub fn at(self, t : f64) -> Vec3 {
              self.origin + t*self.direction
       }

       pub fn through_points(origin : Vec3, target : Vec3) -> Ray {
              Ray { origin: origin, direction: target - origin }
       }
}