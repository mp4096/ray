use crate::color::Color;
use crate::hittable::Face;
use crate::material::{Material, ScatterResult};
use crate::ray::Ray;
use crate::vec3::Vec3;

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
        metal_scatter(incoming_ray, normal, point, &self.albedo, self.fuzz)
    }
}
