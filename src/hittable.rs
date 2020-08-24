use crate::material_variants::MaterialVariants;
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
    pub material: MaterialVariants,
}

pub trait Hittable: Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut res: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if let Some(current_hit) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = current_hit.t;
                res = Some(current_hit);
            }
        }
        res
    }
}
