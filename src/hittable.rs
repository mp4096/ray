use crate::ray::Ray;
use crate::vec3::Vec3;

pub enum Face {
    Inside,
    Outside,
}

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub face: Face,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
