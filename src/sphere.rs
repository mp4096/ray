use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: &Vec3, radius: f64) -> Sphere {
        Sphere {
            center: *center,
            radius,
        }
    }

    pub fn shade_normal(&self, normal_vector: &Vec3) -> Color {
        0.5 * Color::new(
            normal_vector.x + 1.0,
            normal_vector.y + 1.0,
            normal_vector.z + 1.0,
        )
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.squared_length();
        let half_b = oc.dot(&r.direction);
        let c = oc.squared_length() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let discriminant_root = discriminant.sqrt();

        let first_root = (-half_b - discriminant_root) / a;
        if (first_root < t_max) && (first_root > t_min) {
            return Some(HitRecord {
                p: r.at(first_root),
                t: first_root,
                normal: (r.at(first_root) - self.center) / self.radius,
            });
        }

        let second_root = (-half_b + discriminant_root) / a;
        if (second_root < t_max) && (second_root > t_min) {
            return Some(HitRecord {
                p: r.at(second_root),
                t: second_root,
                normal: (r.at(second_root) - self.center) / self.radius,
            });
        }

        None
    }
}
