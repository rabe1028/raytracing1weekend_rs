use crate::vec3::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        &self.origin + &(t * &self.direction)
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }
}

// Ray reflect
pub fn reflect(v: &Vec3, normal: &Vec3) -> Vec3 {
    v - 2. * v.dot(normal) * normal
}

// Ray refract
pub fn refract(uv: &Vec3, normal: &Vec3, eta_i_over_eta_t: f64) -> Vec3 {
    let cos_theta = (-1.0 * uv).dot(normal).min(1.0);
    let r_out_perp = eta_i_over_eta_t * (uv + cos_theta * normal);
    let r_out_parallel = (1.0 - r_out_perp.norm()).abs().sqrt() * -1.0 * normal;
    r_out_perp + r_out_parallel
}
