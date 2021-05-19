use rand::Rng;

use crate::ray::{reflect, refract};
use crate::{hittable::HitRecord, material::Material, ray::Ray, vec3::Color};

fn reflectance(cosine: f64, ref_index: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = (1.0 - ref_index) / (1.0 + ref_index);
    let r0 = r0.powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

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

        let mut rng = rand::thread_rng();

        let direction = if refraction_ratio * sin_theta > 1.0
            || reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..1.0)
        {
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
