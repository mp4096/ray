use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

use indicatif::ProgressBar;
use itertools::iproduct;

mod vec3;
use vec3::Vec3;
mod color;
use color::Color;

mod ray;
use ray::Ray;

fn write_ppm(width: usize, height: usize, pixels: &[Color]) -> std::io::Result<()> {
    let file = File::create("out.ppm")?;
    let mut buf_writer = BufWriter::new(file);
    let header = format!("P6 {} {} 255 ", width, height);
    buf_writer.write_all(header.as_bytes())?;
    for p in pixels {
        buf_writer.write_all(&[p.red, p.green, p.blue])?;
    }
    Ok(())
}

fn main() {
    let width = 1920;
    let height = 1080;
    let total_pixels = width * height;

    let mut vec: Vec<Color> = Vec::with_capacity(width * height);
    let coordinates_range = iproduct!((0..height).rev(), 0..width);
    let pb = ProgressBar::new(total_pixels as u64);
    pb.set_draw_delta((total_pixels / 100) as u64);

    println!("Writing a {}x{} image", width, height);
    for (j, i) in pb.wrap_iter(coordinates_range) {
        let r = (i as f64) / ((width - 1) as f64);
        let g = (j as f64) / ((height - 1) as f64);
        let b = 0.25;

        vec.push(Color::from_rgb_double(r, g, b));
    }
    match write_ppm(width, height, &vec) {
        Ok(_) => println!("Ok!"),
        Err(_) => println!("nok..."),
    }

    let point = Vec3::new(1.0, 2.0, 3.0);
    let neg_point = -point;
    println!("{}", neg_point.x);
    let color = Color::from_rgb_double(1.0, 0.4, 1.0);
    println!("{}", color);
}
