use crate::color::Color;
use crate::material::{Material, ScatterResult};
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Debug, PartialEq, Clone)]
pub struct Metal {
    pub albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, incoming_ray: &Ray, normal: &Vec3, point: &Vec3) -> ScatterResult {
        // let scatter_direction = *normal + Vec3::random_unit_vector();
        // let reflected = reflect(unit_vector(r_in.direction()), rec.normal);

        let v = incoming_ray.direction.make_unit_vector();
        let n = normal;

        let reflected = v - 2.0 * v.dot(n) * *n;

        let scattered = Ray::new(*point, reflected);

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
        }
    }
}
