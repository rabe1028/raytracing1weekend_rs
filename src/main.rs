mod vec3;
use vec3::{Color, Point3, Vec3};

mod color;
mod ray;
use ray::Ray;

mod hittable;
use hittable::Hittable;
mod sphere;
use sphere::Sphere;

mod hittable_list;
use hittable_list::HittableList;

mod camera;
use camera::Camera;

use std::io::{stderr, Write};

use rand::Rng;
use rayon::prelude::*;
use std::sync::Arc;

fn ray_color(r: &Ray, world: &impl Hittable) -> Color {
    if let Some(rec) = world.hit(r, 0., f64::INFINITY) {
        0.5 * (rec.normal + Color::new(1., 1., 1.))
    } else {
        let unit_direction = r.direction().clone().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1. - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.)
    }
}

fn main() {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;
    let sample_per_pixel = 100;

    // World
    let world = HittableList::new()
        .push(Sphere::new(Point3::new(0., 0., -1.), 0.5))
        .push(Sphere::new(Point3::new(0., -100.5, -1.), 100.));

    let world = Arc::new(world);

    // Camera

    let cam = Camera::new();

    let cam = Arc::new(cam);

    // Render

    println!("P3\n{} {}\n255", image_width, image_height);

    let pixel_colors : Vec<Color> = (0..image_height).into_par_iter().rev().map(move |j| {
        eprint!("\rScanlines remaining: {} ", j);
        stderr().flush().unwrap();

        let world = world.clone();
        let cam = cam.clone();

        (0..image_width).into_par_iter().map(move |i| {
            let mut rng = rand::thread_rng();

            let world = world.clone();
            let cam = cam.clone();

            (0..sample_per_pixel).fold(Color::new(0., 0., 0.), |acc, _| {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width as f64 - 1.);
                let v = (j as f64 + rng.gen::<f64>()) / (image_height as f64 - 1.);

                let r = cam.get_ray(u, v);
                acc + ray_color(&r, world.as_ref())
            }) / sample_per_pixel as f64
        })
    }).flatten().collect();

    pixel_colors.iter().for_each(|pixel_color| {println!("{}", pixel_color);});

    eprintln!("\nDone.");
}
