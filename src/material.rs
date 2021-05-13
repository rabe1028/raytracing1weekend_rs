

use crate::hittable::HitRecord;
use crate::Color;
use crate::Ray;

pub trait Material {
    // dynamic objectで管理する関係上、&mut implを受け取れないので、rand関係を利用する場合は、method内で作成すること
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct UninitMaterial {}
impl Material for UninitMaterial {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }
}
