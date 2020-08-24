use crate::dielectric::dielectric_scatter;
use crate::lambertian::lambertian_scatter;
use crate::material::{Material, ScatterResult};
use crate::metal::metal_scatter;

use crate::color::Color;
use crate::hittable::Face;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub enum MaterialVariants {
    Metal(Color, f64),
    Lambertian(Color),
    Dielectric(f64),
}

impl Material for MaterialVariants {
    fn scatter(
        &self,
        incoming_ray: &Ray,
        normal: &Vec3,
        point: &Vec3,
        face: Face,
    ) -> ScatterResult {
        match self {
            MaterialVariants::Metal(albedo,  fuzz) => {
                metal_scatter(incoming_ray, normal, point, albedo, *fuzz)
            }
            MaterialVariants::Lambertian(albedo) => lambertian_scatter(normal, point, albedo),
            MaterialVariants::Dielectric(ref_idx) => {
                dielectric_scatter(incoming_ray, normal, point, face, *ref_idx)
            }
        }
    }
}
