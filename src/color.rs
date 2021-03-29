use crate::vec3::Color;
use std::fmt;

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Divide the color by the number of samples and gamma-correct for gamma=2.0.
        let r = (self.x.sqrt() * 255.999) as u8;
        let g = (self.y.sqrt() * 255.999) as u8;
        let b = (self.z.sqrt() * 255.999) as u8;
        write!(f, "{} {} {}", r, g, b)
    }
}
