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

mod material;
use material::Material;

mod lambertian;
use lambertian::Lambertian;

mod metal;
use metal::Metal;

mod dielectrics;
use dielectrics::Dielectrics;

mod util;

use std::io::{stderr, Write};

use rand::Rng;
use rayon::prelude::*;
use std::sync::Arc;

fn ray_color(r: &Ray, world: &impl Hittable, rng: &mut impl rand::Rng, depth: u64) -> Color {
    if depth == 0 {
        return Color::new(0., 0., 0.);
    }
    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        // let target = &(rec.p) + &(rec.normal).random_in_hemisphere(rng);
        // 0.5 * ray_color(&Ray::new(rec.p.clone(), target - rec.p), world, rng, depth - 1)
        if let Some((attenuation, scattered)) = rec.mat_ptr.scatter(r, &rec) {
            attenuation * ray_color(&scattered, world, rng, depth - 1)
        } else {
            Color::new(0., 0., 0.)
        }
    } else {
        let unit_direction = r.direction().clone().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1. - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.)
    }
}

fn random_scene() -> HittableList {
    let world = HittableList::new();

    let ground_material: Arc<Box<dyn Material + Sync + Send + 'static>> =
        Arc::new(Box::new(Lambertian::new(Color::new(0.5, 0.5, 0.5))));

    let world = world.push(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    let world = (-11..11).zip(-11..11).fold(world, |world, (a, b)| {
        let mut rng = rand::thread_rng();
        let choose_mat = rng.gen_range(0.0..1.0);
        let center = Point3::new(
            a as f64 + 0.9 * rng.gen_range(0.0..1.0),
            0.2,
            b as f64 + 0.9 * rng.gen_range(0.0..1.0),
        );

        if (&center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
            let sphere_material: Box<dyn Material + Sync + Send + 'static> = if choose_mat < 0.8 {
                // diffuse
                let albedo = Color::vec3_random_range(&mut rng, 0.0..1.0);
                Box::new(Lambertian::new(albedo))
            } else if choose_mat < 0.95 {
                // metal
                let albedo = Color::vec3_random_range(&mut rng, 0.0..0.5);
                let fuzz = rng.gen_range(0.0..0.5);
                Box::new(Metal::new(albedo, fuzz))
            } else {
                // glass
                Box::new(Dielectrics::new(1.0))
            };
            world.push(Sphere::new(center, 0.2, Arc::new(sphere_material)))
        } else {
            world
        }
    });

    let material1: Arc<Box<dyn Material + Sync + Send + 'static>> =
        Arc::new(Box::new(Dielectrics::new(1.5)));

    let material2: Arc<Box<dyn Material + Sync + Send + 'static>> =
        Arc::new(Box::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))));

    let material3: Arc<Box<dyn Material + Sync + Send + 'static>> =
        Arc::new(Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)));

    world
        .push(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1))
        .push(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2))
        .push(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3))
}

fn main() {
    // Image

    let aspect_ratio = 3.0 / 2.0;
    let image_width: usize = 1200;
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;
    let sample_per_pixel = 500;
    let max_depth = 50;

    // World

    let world = Arc::new(random_scene());

    // Camera

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let viewup = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        viewup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let cam = Arc::new(cam);

    // Render

    println!("P3\n{} {}\n255", image_width, image_height);

    let pixel_colors: Vec<Color> = (0..image_height)
        .into_par_iter()
        .rev()
        .map(move |j| {
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
                    acc + ray_color(&r, world.as_ref(), &mut rng, max_depth)
                }) / sample_per_pixel as f64
            })
        })
        .flatten()
        .collect();

    pixel_colors.iter().for_each(|pixel_color| {
        println!("{}", pixel_color);
    });

    eprintln!("\nDone.");
}
