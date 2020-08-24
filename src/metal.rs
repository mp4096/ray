use crate::color::Color;
use crate::material::ScatterResult;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[inline]
pub fn metal_scatter(
    incoming_ray: &Ray,
    normal: &Vec3,
    point: &Vec3,
    albedo: &Color,
    fuzz: f64,
) -> ScatterResult {
    let reflected = incoming_ray.direction.make_unit_vector().reflect(normal);

    let scattered = Ray::new(*point, reflected + fuzz * Vec3::random_in_unit_sphere());

    if scattered.direction.dot(normal) > 0.0 {
        ScatterResult::Scattered {
            attenuation: *albedo,
            scattered,
        }
    } else {
        ScatterResult::Absorbed
    }
}
