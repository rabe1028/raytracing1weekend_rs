use crate::vec3::Color;
use std::fmt;

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = (self.x * 255.999) as u8;
        let g = (self.y * 255.999) as u8;
        let b = (self.z * 255.999) as u8;
        write!(f, "{} {} {}", r, g, b)
    }
}
