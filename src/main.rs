use std::io::{stderr, Write};

fn main() {

    // Image

    let image_width : usize = 256;
    let image_height : usize = 256;

    // Render

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        stderr().flush().unwrap();

        for i in 0..image_width {
            let r = i as f64 / (image_width as f64 - 1.);
            let g = j as f64 / (image_height as f64 - 1.);
            let b = 0.25f64;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            println!("{} {} {}", ir, ig, ib);
        }
    }

    eprintln!("\nDone.");
}
