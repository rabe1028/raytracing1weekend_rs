use crate::ray::Ray;
use crate::vec3::*;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        viewup: Vec3,
        vertical_fov: f64,
        aspect_ratio: f64,
        aperture: f64, // 絞り
        focus_dist: f64,
    ) -> Self {
        let theta = crate::util::degrees_to_radians(vertical_fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (&lookfrom - &lookat).normalize();
        let u = viewup.cross(&w).normalize();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * &u;
        let vertical = focus_dist * viewport_height * &v;
        let lower_left_corner = &origin - &horizontal / 2. - &vertical / 2. - focus_dist * &w;

        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let mut rng = rand::thread_rng();
        let rd = self.lens_radius * Vec3::random_in_unit_disk(&mut rng);
        let offset = rd.x * &self.u + rd.y * &self.v;

        Ray::new(
            &self.origin + &offset,
            &(self.lower_left_corner) + u * &(self.horizontal) + v * &(self.vertical)
                - &(self.origin)
                - offset,
        )
    }
}
