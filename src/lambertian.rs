use crate::color::Color;
use crate::material::ScatterResult;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[inline]
pub fn lambertian_scatter(normal: &Vec3, point: &Vec3, albedo: &Color) -> ScatterResult {
    let scatter_direction = *normal + Vec3::random_unit_vector();
    ScatterResult::Scattered {
        attenuation: *albedo,
        scattered: Ray::new(*point, scatter_direction),
    }
}
