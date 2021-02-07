use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r : f64,
    pub g : f64,
    pub b : f64
}

impl Color {
       pub fn from_rgb(r : f64, g : f64, b : f64) -> Color { Color { r, g, b } }
       pub fn to_rgb(self) -> (f64,f64,f64) { (self.r, self.g, self.b) }
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

impl ops::Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, c : Color) -> Color { Color::from_rgb(self * c.r, self * c.g, self * c.b) } 
}

impl ops::Mul<f64> for Color {
    type Output = Color;
    fn mul(self, r : f64) -> Color { r * self } 
}

impl ops::Div<f64> for Color {
       type Output = Color;
       fn div(self, d : f64) -> Color { (1.0/d) * self } 
}