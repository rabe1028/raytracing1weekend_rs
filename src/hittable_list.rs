use crate::hittable::*;
use crate::Ray;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable + Sync + Send + 'static>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn push(mut self, object: impl Hittable + Sync + Send + 'static) -> Self {
        let object = Box::new(object);
        self.objects.push(object);
        self
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let (record, _) =
            self.objects
                .iter()
                .fold((None, t_max), |(rec, closest_so_far), object| {
                    if let Some(v) = object.hit(r, t_min, closest_so_far) {
                        let t = v.t;
                        (Some(v), t)
                    } else {
                        (rec, closest_so_far)
                    }
                });

        record
    }
}
