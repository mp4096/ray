use crate::color::Color;
use crate::hittable::Face;
use crate::material::ScatterResult;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::distributions::{Distribution, Uniform};

pub fn dielectric_scatter(
    incoming_ray: &Ray,
    normal: &Vec3,
    point: &Vec3,
    face: Face,
    ref_idx: f64,
) -> ScatterResult {
    let etai_over_etat = match face {
        Face::Inside => ref_idx,
        Face::Outside => 1.0 / ref_idx,
    };

    let unit_direction = incoming_ray.direction.make_unit_vector();

    let cos_theta = normal.dot(&(-unit_direction)).min(1.0_f64);
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
    if etai_over_etat * sin_theta > 1.0_f64 {
        let reflected = unit_direction.reflect(normal);
        return ScatterResult::Scattered {
            attenuation: Color::new_white(),
            scattered: Ray::new(*point, reflected),
        };
    }
    let reflect_prob = schlick(cos_theta, etai_over_etat);

    let mut rng = rand::thread_rng();
    let uniform_dist = Uniform::new(0.0_f64, 1.0_f64);
    if uniform_dist.sample(&mut rng) < reflect_prob {
        let reflected = unit_direction.reflect(normal);
        return ScatterResult::Scattered {
            attenuation: Color::new_white(),
            scattered: Ray::new(*point, reflected),
        };
    }

    let refracted = refract(&unit_direction, normal, etai_over_etat);

    ScatterResult::Scattered {
        attenuation: Color::new_white(),
        scattered: Ray::new(*point, refracted),
    }
}

fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = -uv.dot(n);

    let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_parallel = -(1.0 - r_out_perp.squared_length()).abs().sqrt() * *n;

    r_out_perp + r_out_parallel
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0_squared = r0 * r0;
    r0_squared + (1.0 - r0_squared) * (1.0 - cosine).powi(5)
}
