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

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec: HitRecord = HitRecord {
            p: Vec3::origin(),
            normal: Vec3::origin(),
            t: 0.0,
            face: Face::Outside,
        };
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            let result = object.hit(r, t_min, closest_so_far);

            match result {
                None => (),
                Some(temp_rec) => {
                    hit_anything = true;
                    closest_so_far = temp_rec.t;
                    rec = temp_rec;
                }
            }
        }

        if hit_anything {
            Some(rec)
        } else {
            None
        }
    }
}
