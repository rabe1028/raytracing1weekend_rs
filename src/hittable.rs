use crate::material::Material;
use crate::vec3::*;
use crate::Ray;
use std::sync::Arc;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: Arc<Box<dyn Material + Sync + Send + 'static>>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn from_face_normal(
        r: &Ray,
        p: Point3,
        outward_normal: &Vec3,
        t: f64,
        mat_ptr: Arc<Box<dyn Material + Sync + Send + 'static>>,
    ) -> HitRecord {
        let front_face = r.direction().dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal.clone()
        } else {
            -outward_normal.clone()
        };
        HitRecord {
            p,
            normal,
            mat_ptr,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
