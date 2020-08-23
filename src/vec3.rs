use rand::distributions::{Distribution, Uniform};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn origin() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn squared_length(&self) -> f64 {
        self.dot(self)
    }

    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self[1] * other[2] - self[2] * other[1],
            y: self[2] * other[0] - self[0] * other[2],
            z: self[0] * other[1] - self[1] * other[0],
        }
    }

    pub fn make_unit_vector(&self) -> Vec3 {
        (*self) / self.length()
    }

    pub fn random() -> Vec3 {
        Vec3::random_with_bounds(-1.0, 1.0)
    }

    pub fn random_with_bounds(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        let uniform_dist = Uniform::new_inclusive(min, max);

        Vec3 {
            x: uniform_dist.sample(&mut rng),
            y: uniform_dist.sample(&mut rng),
            z: uniform_dist.sample(&mut rng),
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_with_bounds(-1.0, 1.0);
            if p.squared_length() <= 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        let mut rng = rand::thread_rng();
        let uniform_a = Uniform::new(0.0_f64, 2.0_f64 * std::f64::consts::PI);
        let uniform_z = Uniform::new_inclusive(-1.0_f64, 1.0_f64);

        let a = uniform_a.sample(&mut rng);
        let z = uniform_z.sample(&mut rng);
        let r = (1.0_f64 - z * z).sqrt();

        Vec3 {
            x: r * a.cos(),
            y: r * a.sin(),
            z,
        }
    }

    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        *self - 2.0 * self.dot(n) * (*n)
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();
        let uniform = Uniform::new(-1.0_f64, 1.0_f64);

        loop {
            let p = Vec3::new(uniform.sample(&mut rng), uniform.sample(&mut rng), 0.0);
            if p.squared_length() >= 1.0 {
                continue;
            };
            return p;
        }
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self += -other
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + (-other)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        other * self
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0_f64 / rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0_f64 / rhs)
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => &f64::NAN,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn length_correct() {
        let vector = Vec3::new(1_f64, 1_f64, 1_f64);
        println!("Vector {}", vector);
        assert_eq!(vector.length(), 1.7320508075688772_f64);
    }

    #[test]
    fn this_too() {
        let vector = Vec3::new(1_f64, 2_f64, 3_f64);
        println!("length {}, {}, {}", vector[0], vector[1], vector[2]);
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn scalar_multiplication_commutes() {
        let vector = Vec3::new(1_f64, 2_f64, 3_f64);
        assert_eq!(2.0_f64 * vector, vector * 2.0_f64);
    }

    #[test]
    fn subtraction() {
        let vector = Vec3::new(1_f64, 2_f64, 3_f64);
        assert_eq!(vector - vector, Vec3::origin());
    }

    #[test]
    fn make_unit() {
        let vector = Vec3::new(1_f64, 2_f64, 3_f64);
        assert_eq!(vector.make_unit_vector().length(), 1.0_f64);
    }

    #[test]
    fn cross() {
        let x_axis = Vec3::new(1.0, 0.0, 0.0);
        let y_axis = Vec3::new(0.0, 1.0, 0.0);
        let expected_z_axis = Vec3::new(0.0, 0.0, 1.0);
        assert_eq!(x_axis.cross(&y_axis), expected_z_axis);
    }
}
