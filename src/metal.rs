use crate::color::Color;
use crate::hittable::Face;
use crate::material::{Material, ScatterResult};
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Debug, PartialEq, Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        incoming_ray: &Ray,
        normal: &Vec3,
        point: &Vec3,
        _face: Face,
    ) -> ScatterResult {
        let v = incoming_ray.direction.make_unit_vector();
        let n = normal;

        let reflected = v - 2.0 * v.dot(n) * *n;

        let scattered = Ray::new(
            *point,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );

        if scattered.direction.dot(normal) > 0.0 {
            ScatterResult::Scattered {
                attenuation: self.albedo,
                scattered,
            }
        } else {
            ScatterResult::Absorbed
        }
    }

    fn default() -> Self {
        Self {
            albedo: Vec3::origin(),
            fuzz: 0.0,
        }
    }
}
