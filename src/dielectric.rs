use crate::color::Color;
use crate::hittable::Face;
use crate::material::{Material, ScatterResult};
use crate::ray::Ray;
use crate::vec3::Vec3;

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

    return r_out_perp + r_out_parallel;
}

impl Material for Dielectric {
    fn scatter(
        &self,
        incoming_ray: &Ray,
        normal: &Vec3,
        point: &Vec3,
        face: Face,
    ) -> ScatterResult {
        let attenuation = Color::new_white();

        let etai_over_etat = match face {
            Face::Inside => self.ref_idx,
            Face::Outside => 1.0 / self.ref_idx,
        };

        let unit_direction = incoming_ray.direction.make_unit_vector();
        let refracted = refract(&unit_direction, normal, etai_over_etat);
        let scattered = Ray::new(*point, refracted);

        ScatterResult::Scattered {
            attenuation,
            scattered,
        }
    }

    fn default() -> Self {
        Self { ref_idx: 0.0 }
    }
}
