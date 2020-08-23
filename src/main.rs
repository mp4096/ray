use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

use indicatif::ProgressBar;
use itertools::iproduct;
use rand::distributions::{Distribution, Uniform};

mod camera;
mod color;
mod hittable;
mod ray;
mod sphere;
mod util;
mod vec3;

use camera::Camera;
use color::Color;
use hittable::{Hittable, HittableList};
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

fn sky_color(r: &Ray) -> Color {
    let unit_direction = r.direction.make_unit_vector();
    let t = 0.5 * unit_direction.y + 1.0_f64;
    (1.0_f64 - t) * Color::new_white() + t * Color::new(0.5, 0.7, 1.0)
}

fn ray_color<T: Hittable>(r: &Ray, world: &T) -> Color {
    match world.hit(r, 0.0, f64::INFINITY) {
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
    let mut rng = rand::thread_rng();
    let uniform_dist = Uniform::new_inclusive(-0.5_f64, 0.5_f64);

    let width = 1920;
    let height = 1080;
    let aspect_ratio = (width as f64) / (height as f64);
    let total_pixels = width * height;

    let mut vec: Vec<Color> = Vec::with_capacity(width * height);
    let coordinates_range = iproduct!((0..height).rev(), 0..width);
    let pb = ProgressBar::new(total_pixels as u64);
    pb.set_draw_delta((total_pixels / 100) as u64);

    // Camera
    let camera = Camera::default(aspect_ratio);

    // Scene
    let mut scene = HittableList::new();

    scene.add(Box::new(Sphere::new(&Vec3::new(1.0, 1.0, -1.0), 0.1)));

    scene.add(Box::new(Sphere::new(&Vec3::new(-1.9, 0.0, -5.0), 1.0)));
    scene.add(Box::new(Sphere::new(&Vec3::new(-0.7, 0.5, -4.0), 0.2)));

    scene.add(Box::new(Sphere::new(&Vec3::new(0.9, 0.2, -5.0), 1.0)));
    scene.add(Box::new(Sphere::new(&Vec3::new(1.7, -0.2, -4.0), 0.5)));

    // Ground
    scene.add(Box::new(Sphere::new(&Vec3::new(0.0, -202.0, -1.0), 200.0)));

    let samples_per_pixel: usize = 10;

    println!("Writing a {}x{} image", width, height);
    for (j, i) in pb.wrap_iter(coordinates_range) {
        let pixel_color = std::iter::repeat_with(|| {
            (
                (i as f64 + uniform_dist.sample(&mut rng)) / ((width - 1) as f64),
                (j as f64 + uniform_dist.sample(&mut rng)) / ((height - 1) as f64),
            )
        })
        .take(samples_per_pixel)
        .map(|uv| camera.get_ray(uv.0, uv.1))
        .map(|r| ray_color(&r, &scene))
        .fold(Vec3::origin(), |acc, c| acc + c)
            / (samples_per_pixel as f64);

        vec.push(pixel_color);
    }

    match write_ppm(width, height, &vec) {
        Ok(_) => println!("Ok!"),
        Err(_) => println!("nok..."),
    }
}
