use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub enum Face {
    Inside,
    Outside,
}

pub struct HitRecord<T: Material + Copy> {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub face: Face,
    pub material: T,
}

pub trait Hittable<T: Material + Copy> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<T>>;
}

pub struct HittableList<T: Material + Copy> {
    pub objects: Vec<Box<dyn Hittable<T>>>,
}

impl<T: Material + Copy> HittableList<T> {
    pub fn new() -> HittableList<T> {
        HittableList::<T> {
            objects: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable<T>>) {
        self.objects.push(object);
    }
}

impl<T: Material + Copy> Hittable<T> for HittableList<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<T>> {
        let mut rec = HitRecord::<T> {
            p: Vec3::origin(),
            normal: Vec3::origin(),
            t: 0.0,
            face: Face::Outside,
            material: T::default(),
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
