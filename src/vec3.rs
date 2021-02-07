use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x : f64,
    pub y : f64,
    pub z : f64
}

pub fn vec3(x : f64, y : f64, z : f64) -> Vec3 { Vec3 { x, y, z } }

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, other : Vec3) -> Vec3 { vec3(self.x + other.x, self.y + other.y, self.z + other.z) }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, other : Vec3) -> Vec3 { vec3(self.x - other.x, self.y - other.y, self.z - other.z) }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, v : Vec3) -> Vec3 { vec3(self * v.x, self * v.y, self * v.z) } 
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, r : f64) -> Vec3 { r * self } 
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = f64;
    fn mul(self, other : Vec3) -> f64 { self.x*other.x + self.y*other.y + self.z*other.z } 
}

impl ops::Div<f64> for Vec3 {
       type Output = Vec3;
       fn div(self, d : f64) -> Vec3 { (1.0/d) * self } 
}

impl Vec3 {
    pub fn length_squared(self : Vec3) -> f64 { self * self }
    pub fn length(self : Vec3) -> f64 { self.length_squared().sqrt() }
    pub const ZERO : Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

    pub fn cross(a : Vec3, b : Vec3) -> Vec3 {
       vec3(a.y*b.z - a.z*b.y,
            a.z*b.x - a.x*b.z,
            a.x*b.y - a.y*b.x)
    }

    pub fn unit(self) -> Vec3 {
       self / self.length()
    }
}