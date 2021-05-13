use crate::ray::{reflect, refract};
use crate::{hittable::HitRecord, material::Material, ray::Ray, vec3::Color};

pub struct Dielectrics {
    index_of_refraction: f64,
}

impl Dielectrics {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }
}

impl Material for Dielectrics {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = r_in.direction().clone().normalize();

        let cos_theta = (-1.0 * &unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let direction = if refraction_ratio * sin_theta > 1.0 {
            // Reflect
            reflect(&unit_direction, &rec.normal)
        } else {
            // Refract
            refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        let scattered = Ray::new(rec.p.clone(), direction);

        Some((attenuation, scattered))
    }
}
