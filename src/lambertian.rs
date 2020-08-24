use crate::color::Color;
use crate::hittable::Face;
use crate::material::{Material, ScatterResult};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub fn lambertian_scatter(normal: &Vec3, point: &Vec3, albedo: &Color) -> ScatterResult {
    let scatter_direction = *normal + Vec3::random_unit_vector();
    ScatterResult::Scattered {
        attenuation: *albedo,
        scattered: Ray::new(*point, scatter_direction),
    }
}

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
    fn scatter(
        &self,
        _incoming_ray: &Ray,
        normal: &Vec3,
        point: &Vec3,
        _face: Face,
    ) -> ScatterResult {
        lambertian_scatter(normal, point, &self.albedo)
    }
}
