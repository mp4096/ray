use crate::lambertian::Lambertian;
use crate::material::{Material, ScatterResult};
use crate::metal::Metal;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub enum MaterialVariants {
    Metal(Metal),
    Lambertian(Lambertian),
}

impl Material for MaterialVariants {
    fn scatter(&self, incoming_ray: &Ray, normal: &Vec3, point: &Vec3) -> ScatterResult {
        match self {
            MaterialVariants::Metal(m) => m.scatter(incoming_ray, normal, point),
            MaterialVariants::Lambertian(m) => m.scatter(incoming_ray, normal, point),
        }
    }

    fn default() -> Self {
        MaterialVariants::Lambertian(Lambertian::default())
    }
}
