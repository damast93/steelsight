use crate::*;
use std::ops;


#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x : float,
    pub y : float,
    pub z : float
}

pub fn vec3(x : float, y : float, z : float) -> Vec3 { Vec3 { x, y, z } }

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, other : Vec3) -> Vec3 { vec3(self.x + other.x, self.y + other.y, self.z + other.z) }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, other : Vec3) -> Vec3 { vec3(self.x - other.x, self.y - other.y, self.z - other.z) }
}

impl ops::Mul<Vec3> for float {
    type Output = Vec3;
    fn mul(self, v : Vec3) -> Vec3 { vec3(self * v.x, self * v.y, self * v.z) } 
}

impl ops::Mul<float> for Vec3 {
    type Output = Vec3;
    fn mul(self, r : float) -> Vec3 { r * self } 
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = float;
    fn mul(self, other : Vec3) -> float { self.x*other.x + self.y*other.y + self.z*other.z } 
}

impl ops::Div<float> for Vec3 {
       type Output = Vec3;
       fn div(self, d : float) -> Vec3 { (1.0/d) * self } 
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 { (-1.0) * self }
}

impl Vec3 {
    pub fn length_squared(self : Vec3) -> float { self * self }
    pub fn length(self : Vec3) -> float { self.length_squared().sqrt() }
    pub const ZERO : Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

    pub fn cross(self : Vec3, b : Vec3) -> Vec3 {
        let a = self;
        vec3(a.y*b.z - a.z*b.y,
             a.z*b.x - a.x*b.z,
             a.x*b.y - a.y*b.x)
    }

    pub fn unit(self) -> Vec3 {
       self / self.length()
    }

    pub fn near_zero(self) -> bool {
        const E: float = 1e-8;
        (self.x.abs() < E) & (self.y.abs() < E) & (self.z.abs() < E) 
    }
}