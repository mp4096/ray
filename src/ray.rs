// mod vec3;
use crate::Vec3;

#[derive(Debug)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: &Vec3, direction: &Vec3) -> Ray {
        Ray {
            origin: *origin,
            direction: *direction,
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::Ray;
    use super::Vec3;

    #[test]
    fn construct() {
        let origin = Vec3::new(1_f64, 2_f64, 3_f64);
        let direction = Vec3::new(4_f64, 5_f64, 6_f64);

        let ray = Ray::new(&origin, &direction);

        println!("ray {:?}", ray);
    }

    #[test]
    fn at_ok() {
        let origin = Vec3::new(1_f64, 2_f64, 3_f64);
        let direction = Vec3::new(1_f64, 0_f64, 0_f64);

        let ray = Ray::new(&origin, &direction);

        let at = ray.at(10_f64);

        println!("at {:?}", at);
        assert_eq!(at, Vec3::new(11_f64, 2_f64, 3_f64));
    }
}
