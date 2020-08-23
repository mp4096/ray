use crate::color::Color;
use crate::material::{Material, ScatterResult};
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Debug, PartialEq, Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _incoming_ray: &Ray, normal: &Vec3, point: &Vec3) -> ScatterResult {
        let scatter_direction = *normal + Vec3::random_unit_vector();
        ScatterResult::Scattered {
            attenuation: self.albedo,
            scattered: Ray::new(*point, scatter_direction),
        }
    }

    fn default() -> Self {
        Self {
            albedo: Vec3::origin(),
        }
    }
}
