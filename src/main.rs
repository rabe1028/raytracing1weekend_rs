mod vec3;
use vec3::{Color, Point3, Vec3};

mod color;
mod ray;
use ray::Ray;

use std::io::{stderr, Write};

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin() - center;
    let a = Vec3::dot(r.direction(), r.direction());
    let b = 2.0 * oc.dot(r.direction());
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4. * a * c;
    (discriminant > 0.)
}

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(&Point3::new(0., 0., -1.0), 0.5, r) {
        return Color::new(1., 0., 0.);
    }
    let unit_direction = r.direction().clone().normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1. - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.)
}

fn main() {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0., 0., 0.);
    let horizontal = Vec3::new(viewport_width, 0., 0.);
    let vertical = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner =
        &origin - &horizontal / 2. - &vertical / 2. - Vec3::new(0., 0., focal_length);

    // Render

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        stderr().flush().unwrap();

        for i in 0..image_width {
            let u = i as f64 / (image_width as f64 - 1.);
            let v = j as f64 / (image_height as f64 - 1.);

            let r = Ray::new(
                origin.clone(),
                &lower_left_corner + u * &horizontal + v * &vertical - &origin,
            );

            let pixel_color = ray_color(&r);

            println!("{}", pixel_color);
        }
    }

    eprintln!("\nDone.");
}
