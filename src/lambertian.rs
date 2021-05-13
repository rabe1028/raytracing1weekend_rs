use crate::hittable::HitRecord;
use crate::material::Material;
use crate::Color;
use crate::Ray;
use crate::Vec3;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = &rec.normal + Vec3::random_unit_vector(&mut rand::thread_rng());

        // Catch degenerate scatter direction
        if (scatter_direction.near_zero()) {
            scatter_direction = rec.normal.clone();
        }

        let scattered = Ray::new(rec.p.clone(), scatter_direction);
        let attenuation = self.albedo.clone();

        Some((attenuation, scattered))
    }
}
