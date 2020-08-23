use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub enum ScatterResult {
    Absorbed,
    Scattered { attenuation: Color, scattered: Ray },
}

pub trait Material {
    fn scatter(&self, incoming_ray: &Ray, normal: &Vec3, point: &Vec3) -> ScatterResult;

    fn default() -> Self;
}
