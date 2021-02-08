use crate::*;
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r : float,
    pub g : float,
    pub b : float
}

impl Color {
       pub fn from_rgb(r : float, g : float, b : float) -> Color { Color { r, g, b } }
       pub fn to_rgb(self) -> (float,float,float) { (self.r, self.g, self.b) }
       pub fn to_rgb_bytes(self) -> (u8,u8,u8) {
              ((255.999 * self.r) as u8,
               (255.999 * self.g) as u8,
               (255.999 * self.b) as u8)
       }
}

impl ops::Add<Color> for Color {
    type Output = Color;
    fn add(self, other : Color) -> Color { Color::from_rgb(self.r + other.r, self.g + other.g, self.b + other.b) }
}

impl ops::Sub<Color> for Color {
    type Output = Color;
    fn sub(self, other : Color) -> Color { Color::from_rgb(self.r - other.r, self.g - other.g, self.b - other.b) }
}

impl ops::Mul<Color> for float {
    type Output = Color;
    fn mul(self, c : Color) -> Color { Color::from_rgb(self * c.r, self * c.g, self * c.b) } 
}

impl ops::Mul<float> for Color {
    type Output = Color;
    fn mul(self, r : float) -> Color { r * self } 
}

impl ops::Div<float> for Color {
       type Output = Color;
       fn div(self, d : float) -> Color { (1.0/d) * self } 
}