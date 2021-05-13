use core::f64;

use crate::hittable::HitRecord;
use crate::material::Material;
use crate::Color;
use crate::Ray;
use crate::Vec3;

use crate::ray::reflect;

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, f: f64) -> Self {
        Self {
            albedo,
            fuzz: if f < 1. { f } else { 1. },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut rng = rand::thread_rng();
        let reflected = reflect(&(r_in.direction().clone().normalize()), &rec.normal);
        let scattered = Ray::new(
            rec.p.clone(),
            reflected + self.fuzz * Vec3::random_in_unit_sphere(&mut rng),
        );
        let attenuation = self.albedo.clone();
        if scattered.direction().dot(&rec.normal) > 0. {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
