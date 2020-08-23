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
mod hittable;
use hittable::{Hittable, HittableList};
mod sphere;
use sphere::Sphere;
mod util;

fn sky_color(r: &Ray) -> Color {
    let unit_direction = r.direction.make_unit_vector();
    let t = 0.5 * unit_direction.y + 1.0_f64;
    (1.0_f64 - t) * Color::new_white() + t * Color::new(0.5, 0.7, 1.0)
}

fn ray_color(r: &Ray) -> Color {
    let mut list = HittableList::new();

    list.add(Box::new(Sphere::new(&Vec3::new(1.0, 1.0, -1.0), 0.1)));
    list.add(Box::new(Sphere::new(&Vec3::new(-1.0, -1.0, -1.0), 0.2)));

    list.add(Box::new(Sphere::new(&Vec3::new(-1.9, 0.0, -5.0), 1.0)));
    list.add(Box::new(Sphere::new(&Vec3::new(-0.7, 0.5, -4.0), 0.2)));

    list.add(Box::new(Sphere::new(&Vec3::new(0.9, 0.2, -5.0), 1.0)));
    list.add(Box::new(Sphere::new(&Vec3::new(1.7, -0.2, -4.0), 0.5)));

    match list.hit(r, 0.0, f64::INFINITY) {
        Some(hit_record) => shade_normal(&hit_record.normal),
        None => sky_color(&r),
    }
}

fn shade_normal(normal_vector: &Vec3) -> Color {
    0.5 * Color::new(
        normal_vector.x + 1.0,
        normal_vector.y + 1.0,
        normal_vector.z + 1.0,
    )
}

fn write_ppm(width: usize, height: usize, pixels: &[Color]) -> std::io::Result<()> {
    let file = File::create("out.ppm")?;
    let mut buf_writer = BufWriter::new(file);
    let header = format!("P6 {} {} 255 ", width, height);
    buf_writer.write_all(header.as_bytes())?;
    for p in pixels {
        buf_writer.write_all(&p.as_bytes())?;
    }
    Ok(())
}

fn main() {
    let width = 1920;
    let height = 1080;
    let aspect_ratio = (width as f64) / (height as f64);
    let total_pixels = width * height;

    let mut vec: Vec<Color> = Vec::with_capacity(width * height);
    let coordinates_range = iproduct!((0..height).rev(), 0..width);
    let pb = ProgressBar::new(total_pixels as u64);
    pb.set_draw_delta((total_pixels / 100) as u64);

    // Camera
    let viewport_height = 2.0_f64;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1_f64;

    let origin = Vec3::origin();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    println!("Writing a {}x{} image", width, height);
    for (j, i) in pb.wrap_iter(coordinates_range) {
        let u = (i as f64) / ((width - 1) as f64);
        let v = (j as f64) / ((height - 1) as f64);

        let ray = Ray::new(
            origin,
            lower_left_corner + u * horizontal + v * vertical - origin,
        );

        let pixel_color = ray_color(&ray);

        vec.push(pixel_color);
    }

    match write_ppm(width, height, &vec) {
        Ok(_) => println!("Ok!"),
        Err(_) => println!("nok..."),
    }
}
