

use crate::material::Material;
use crate::Color;
use crate::Vec3;
use crate::Ray;
use crate::hittable::HitRecord;


fn reflect(v: &Vec3, normal: &Vec3) -> Vec3 {
    v - 2. * v.dot(normal) * normal
}

pub struct Metal {
    albedo : Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self {albedo}
    }
}

impl Material for Metal {
    
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> { 
        let reflected = reflect(&(r_in.direction().clone().normalize()), &rec.normal);
        let scattered = Ray::new(rec.p.clone(), reflected);
        let attenuation = self.albedo.clone();
        if scattered.direction().dot(&rec.normal) > 0. {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}