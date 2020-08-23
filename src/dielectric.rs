use crate::color::Color;
use crate::hittable::Face;
use crate::material::{Material, ScatterResult};
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::distributions::{Distribution, Uniform};

#[derive(Copy, Debug, PartialEq, Clone)]
pub struct Dielectric {
    pub ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Dielectric {
        Dielectric { ref_idx }
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

impl Material for Dielectric {
    fn scatter(
        &self,
        incoming_ray: &Ray,
        normal: &Vec3,
        point: &Vec3,
        face: Face,
    ) -> ScatterResult {
        let etai_over_etat = match face {
            Face::Inside => self.ref_idx,
            Face::Outside => 1.0 / self.ref_idx,
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

    fn default() -> Self {
        Self { ref_idx: 0.0 }
    }
}
