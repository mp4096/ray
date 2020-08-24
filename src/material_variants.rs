use crate::dielectric::Dielectric;
use crate::lambertian::Lambertian;
use crate::material::{Material, ScatterResult};
use crate::metal::Metal;

use crate::hittable::Face;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub enum MaterialVariants {
    Metal(Metal),
    Lambertian(Lambertian),
    Dielectric(Dielectric),
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
            MaterialVariants::Metal(m) => m.scatter(incoming_ray, normal, point, face),
            MaterialVariants::Lambertian(m) => m.scatter(incoming_ray, normal, point, face),
            MaterialVariants::Dielectric(m) => m.scatter(incoming_ray, normal, point, face),
        }
    }
}
