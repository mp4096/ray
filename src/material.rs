use crate::color::Color;
use crate::hittable::Face;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub enum ScatterResult {
    Absorbed,
    Scattered { attenuation: Color, scattered: Ray },
}

pub trait Material: Copy {
    fn scatter(&self, incoming_ray: &Ray, normal: &Vec3, point: &Vec3, face: Face)
        -> ScatterResult;
}
