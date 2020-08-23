use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

use indicatif::ProgressBar;
use itertools::iproduct;
use rand::distributions::{Distribution, Uniform};

mod camera;
mod color;
mod hittable;
mod lambertian;
mod material;
mod material_variants;
mod metal;
mod ray;
mod sphere;
mod util;
mod vec3;

use camera::Camera;
use color::Color;
use hittable::{Hittable, HittableList};
use lambertian::Lambertian;
use material::{Material, ScatterResult};
use material_variants::MaterialsVariants;
use metal::Metal;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

fn sky_color(r: &Ray) -> Color {
    let unit_direction = r.direction.make_unit_vector();
    let t = 0.5 * unit_direction.y + 1.0_f64;
    (1.0_f64 - t) * Color::new_white() + t * Color::new(0.5, 0.7, 1.0)
}

fn ray_color<T: Material, U: Hittable<T>>(r: &Ray, world: &U, depth: isize) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::new_black();
    }

    match world.hit(r, 0.001, f64::INFINITY) {
        Some(hit_record) => {
            match hit_record
                .material
                .scatter(r, &hit_record.normal, &hit_record.p)
            {
                ScatterResult::Scattered {
                    attenuation,
                    scattered,
                } => attenuation * ray_color(&scattered, world, depth - 1),
                ScatterResult::Absorbed => Color::new_black(),
            }
        }
        None => sky_color(&r),
    }
}

#[allow(dead_code)]
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

    let material_ground = MaterialsVariants::Lambertian(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = MaterialsVariants::Lambertian(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_metal = MaterialsVariants::Metal(Metal::new(Color::new(0.8, 0.8, 0.8)));

    // Scene
    let mut scene = HittableList::new();

    scene.add(Box::new(Sphere::new(
        Vec3::new(1.0, 1.0, -1.0),
        0.1,
        material_ground,
    )));

    scene.add(Box::new(Sphere::new(
        Vec3::new(-1.9, 0.0, -5.0),
        1.0,
        material_ground,
    )));
    scene.add(Box::new(Sphere::new(
        Vec3::new(-0.7, 0.5, -4.0),
        0.2,
        material_ground,
    )));

    scene.add(Box::new(Sphere::new(
        Vec3::new(0.9, 0.2, -5.0),
        1.0,
        material_metal,
    )));

    scene.add(Box::new(Sphere::new(
        Vec3::new(1.7, -0.2, -4.0),
        0.5,
        material_center,
    )));

    // Ground
    scene.add(Box::new(Sphere::new(
        Vec3::new(0.0, -202.0, -1.0),
        200.0,
        material_ground,
    )));

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
        .map(|r| ray_color(&r, &scene, 50))
        .fold(Vec3::origin(), |acc, c| acc + c)
            / (samples_per_pixel as f64);

        vec.push(pixel_color.gamma_correction(2.0));
    }

    match write_ppm(width, height, &vec) {
        Ok(_) => println!("Ok!"),
        Err(_) => println!("nok..."),
    }
}
