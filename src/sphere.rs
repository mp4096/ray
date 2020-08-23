use crate::hittable::{Face, HitRecord, Hittable};
use crate::material_variants::MaterialVariants;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: MaterialVariants,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: MaterialVariants) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

#[inline]
fn get_face_normal(r: &Ray, outward_normal: &Vec3) -> (Face, Vec3) {
    let face = if r.direction.dot(outward_normal) < 0.0 {
        Face::Outside
    } else {
        Face::Inside
    };
    let normal = match face {
        Face::Outside => *outward_normal,
        Face::Inside => -(*outward_normal),
    };
    (face, normal)
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
            let outward_normal = (r.at(first_root) - self.center) / self.radius;
            let (face, normal) = get_face_normal(r, &outward_normal);
            return Some(HitRecord {
                p: r.at(first_root),
                t: first_root,
                normal,
                face,
                material: self.material,
            });
        }

        let second_root = (-half_b + discriminant_root) / a;
        if (second_root < t_max) && (second_root > t_min) {
            let outward_normal = (r.at(second_root) - self.center) / self.radius;
            let (face, normal) = get_face_normal(r, &outward_normal);
            return Some(HitRecord {
                p: r.at(second_root),
                t: second_root,
                normal,
                face,
                material: self.material,
            });
        }

        None
    }
}
