use crate::color::Color;
use crate::hittable::Hittable;
use crate::material::{Material, ScatterResult};
use crate::ray::Ray;

fn sky_color(r: &Ray) -> Color {
    let unit_direction = r.direction.make_unit_vector();
    let t = 0.5 * unit_direction.y + 1.0_f64;
    (1.0_f64 - t) * Color::new_white() + t * Color::new(0.5, 0.7, 1.0)
}

pub fn ray_color<T: Hittable>(r: &Ray, world: &T, depth: isize) -> Color {
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

#[cfg(test)]
mod tests {
    use std::time::Instant;

    #[test]
    #[ignore]
    fn stupid_benchmark() {
        use crate::color::Color;
        use crate::hittable::HittableList;
        use crate::material_variants::MaterialVariants;
        use crate::ray::Ray;
        use crate::sphere::Sphere;
        use crate::vec3::Vec3;

        let mut scene = HittableList::new();
        scene.add(Box::new(Sphere::new(
            Vec3::new(-0.05, 0.05, -1.0),
            0.5,
            MaterialVariants::Dielectric(1.5),
        )));
        scene.add(Box::new(Sphere::new(
            Vec3::new(0.05, 0.05, -2.0),
            0.5,
            MaterialVariants::Dielectric(2.5),
        )));
        scene.add(Box::new(Sphere::new(
            Vec3::new(-0.05, 0.05, -2.0),
            0.5,
            MaterialVariants::Dielectric(2.5),
        )));
        scene.add(Box::new(Sphere::new(
            Vec3::new(0.05, 0.05, -3.0),
            0.5,
            MaterialVariants::Dielectric(2.5),
        )));
        scene.add(Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, -4.0),
            0.6,
            MaterialVariants::Metal(Color::new(0.8, 0.6, 0.2), 0.1),
        )));
        // Large sphere behind
        scene.add(Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, 1.0),
            10.0,
            MaterialVariants::Metal(Color::new(0.8, 0.6, 0.2), 0.1),
        )));

        let ray = Ray::new(Vec3::origin(), Vec3::new(0.0, 0.0, 1.0));
        let num_iter = 1_000_000;
        let max_depth = 10;
        let tic = Instant::now();
        let mut acc = Color::new_black();
        for _ in 0..num_iter {
            acc += super::ray_color(&ray, &scene, max_depth);
        }
        let toc = Instant::now();
        println!(
            "Duration {:?} ({} iterations)",
            toc.checked_duration_since(tic),
            num_iter
        );
    }
}
