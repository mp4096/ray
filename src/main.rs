use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

use chrono::prelude::*;

use indicatif::ProgressBar;
use itertools::iproduct;
use rand::distributions::{Distribution, Uniform};

mod camera;
mod color;
mod dielectric;
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
use dielectric::Dielectric;
use hittable::{Hittable, HittableList};
use lambertian::Lambertian;
use material::{Material, ScatterResult};
use material_variants::MaterialVariants;
use metal::Metal;
use ray::Ray;
use rayon::prelude::*;
use sphere::Sphere;
use vec3::Vec3;

fn sky_color(r: &Ray) -> Color {
    let unit_direction = r.direction.make_unit_vector();
    let t = 0.5 * unit_direction.y + 1.0_f64;
    (1.0_f64 - t) * Color::new_white() + t * Color::new(0.5, 0.7, 1.0)
}

fn ray_color<T: Hittable>(r: &Ray, world: &T, depth: isize) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::new_black();
    }

    match world.hit(r, 0.001, f64::INFINITY) {
        Some(hit_record) => {
            match hit_record
                .material
                .scatter(r, &hit_record.normal, &hit_record.p, hit_record.face)
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
    let now = Local::now();

    let file = File::create(format!(
        "{}{}{}_{:02}{:02}{:02}_out.ppm",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second()
    ))?;
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

    // let mut vec: Vec<Color> = Vec::with_capacity(width * height);
    let coordinates_range = iproduct!((0..height).rev(), 0..width);
    // let pb = ProgressBar::new(total_pixels as u64);
    // pb.set_draw_delta((total_pixels / 100) as u64);

    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);

    // Camera
    let camera = Camera::default(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        20.0_f64,
        aspect_ratio,
        0.1,
        10.0,
    );

    let scene = random_scene();

    let samples_per_pixel: usize = 500;

    println!("Writing a {}x{} image", width, height);
    let coordinates_vec: Vec<(usize, usize)> = coordinates_range.collect();
    let vec: Vec<Color> = coordinates_vec
        .par_iter()
        .map(|(j, i)| {
            let mut rng = rand::thread_rng();
            let uniform_dist = Uniform::new_inclusive(-0.5_f64, 0.5_f64);
            (std::iter::repeat_with(|| {
                (
                    (*i as f64 + uniform_dist.sample(&mut rng)) / ((width - 1) as f64),
                    (*j as f64 + uniform_dist.sample(&mut rng)) / ((height - 1) as f64),
                )
            })
            .take(samples_per_pixel)
            .map(|uv| camera.get_ray(uv.0, uv.1))
            .map(|r| ray_color(&r, &scene, 50))
            .fold(Vec3::origin(), |acc, c| acc + c)
                / (samples_per_pixel as f64))
                .gamma_correction(2.0)
        })
        .collect();

    // for (j, i) in pb.wrap_iter(coordinates_range) {
    //     let pixel_color = std::iter::repeat_with(|| {
    //         (
    //             (i as f64 + uniform_dist.sample(&mut rng)) / ((width - 1) as f64),
    //             (j as f64 + uniform_dist.sample(&mut rng)) / ((height - 1) as f64),
    //         )
    //     })
    //     .take(samples_per_pixel)
    //     .map(|uv| camera.get_ray(uv.0, uv.1))
    //     .map(|r| ray_color(&r, &scene, 50))
    //     .fold(Vec3::origin(), |acc, c| acc + c)
    //         / (samples_per_pixel as f64);
    //
    //     vec.push(pixel_color.gamma_correction(2.0));
    // }

    match write_ppm(width, height, &vec) {
        Ok(_) => println!("Ok!"),
        Err(_) => println!("nok..."),
    }
}

fn old_world() -> HittableList {
    let material_ground = MaterialVariants::Lambertian(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = MaterialVariants::Lambertian(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_metal1 = MaterialVariants::Metal(Metal::new(Color::new(0.8, 0.8, 0.8), 0.0));
    let material_metal2 = MaterialVariants::Metal(Metal::new(Color::new(0.8, 0.8, 0.8), 0.1));
    let material_metal3 = MaterialVariants::Metal(Metal::new(Color::new(0.8, 0.6, 0.2), 0.1));

    let material_dielectrical = MaterialVariants::Dielectric(Dielectric::new(1.5));

    // Scene
    let mut scene = HittableList::new();

    scene.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.5, -1.0),
        0.1,
        material_center,
    )));

    scene.add(Box::new(Sphere::new(
        Vec3::new(-1.9, 0.0, -5.0),
        1.0,
        material_metal1,
    )));
    scene.add(Box::new(Sphere::new(
        Vec3::new(-0.7, 0.5, -4.0),
        0.2,
        material_center,
    )));

    scene.add(Box::new(Sphere::new(
        Vec3::new(0.9, 0.2, -5.0),
        1.0,
        material_metal3,
    )));

    scene.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -3.0),
        0.5,
        material_dielectrical,
    )));
    scene.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -3.0),
        -0.4,
        material_dielectrical,
    )));

    scene.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.4, -3.0),
        0.75,
        material_dielectrical,
    )));

    scene.add(Box::new(Sphere::new(
        Vec3::new(1.7, -0.2, -4.0),
        0.5,
        material_metal2,
    )));

    // Ground
    scene.add(Box::new(Sphere::new(
        Vec3::new(0.0, -202.0, -1.0),
        200.0,
        material_ground,
    )));

    scene
}

fn random_scene() -> HittableList {
    let mut world: HittableList = HittableList::new();

    let mut rng = rand::thread_rng();
    let uniform_dist = Uniform::new_inclusive(0.0, 1.0);
    let uniform_dist_0_5 = Uniform::new_inclusive(0.0, 0.5);

    let material_ground = MaterialVariants::Lambertian(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let rand = uniform_dist.sample(&mut rng);
            let center = Vec3::new(
                a as f64 + 0.9 * uniform_dist.sample(&mut rng),
                0.2,
                b as f64 + 0.9 * uniform_dist.sample(&mut rng),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if rand < 0.8 {
                    let albedo: Color = Vec3::random();
                    let material = MaterialVariants::Lambertian(Lambertian::new(albedo));
                    world.add(Box::new(Sphere::new(center, 0.2, material)));
                } else if rand < 0.95 {
                    let albedo: Color = Vec3::random_with_bounds(0.0, 0.5);
                    let fuzz = uniform_dist_0_5.sample(&mut rng);
                    let material = MaterialVariants::Metal(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, material)));
                } else {
                    let material = MaterialVariants::Dielectric(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        MaterialVariants::Dielectric(Dielectric::new(1.5)),
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        MaterialVariants::Lambertian(Lambertian::new(Color::new(0.4, 0.2, 1.0))),
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        MaterialVariants::Metal(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
    )));

    world
}
