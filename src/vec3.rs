use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

use rand::{thread_rng, Rng};

pub type Point3 = Vec3;
pub type Rgb = Vec3;

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub const fn from_scalar(v: f64) -> Self {
        Self { x: v, y: v, z: v }
    }

    pub fn random() -> Self {
        let mut rng = thread_rng();
        Self {
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen(),
        }
    }

    pub fn random_in_range(min: f64, max: f64) -> Self {
        let mut rng = thread_rng();
        Self {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            z: rng.gen_range(min..max),
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let v = Self::random_in_range(-1.0, 1.0);
            if v.length_squared() <= 1.0 {
                return v;
            }
        }
    }

    pub fn random_in_unit_disk() -> Self {
        let mut rng = thread_rng();
        loop {
            let v = Self {
                x: rng.gen_range(-1.0..1.0),
                y: rng.gen_range(-1.0..1.0),
                z: 0.0,
            };
            if v.length_squared() <= 1.0 {
                return v;
            }
        }
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(*self)
    }

    pub fn unit(&self) -> Self {
        *self / self.length()
    }

    pub fn sqrt(&self) -> Self {
        Self {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
            z: self.z.sqrt(),
        }
    }

    pub fn as_bytes(&self) -> [u8; 3] {
        [
            (self.x.clamp(0.0, 0.999) * 256.0) as u8,
            (self.y.clamp(0.0, 0.999) * 256.0) as u8,
            (self.z.clamp(0.0, 0.999) * 256.0) as u8,
        ]
    }

    pub fn near_zero(&self) -> bool {
        let eps = 1e-8;
        self.x.abs() < eps && self.y.abs() < eps && self.z.abs() < eps
    }

    pub fn reflect_across(&self, normal: Vec3) -> Self {
        /*   N
          \  ^  /^
          s\ | / |b
        ____\|/__|__
              \  ^
               \ |b
                \|
        where s is going into the hit point
        flip s (minus sign) then add 2b in dir of normal (unit), hence get reflected ray (going out from point)
        */
        let b_length = self.dot(normal);
        *self - normal * 2.0 * b_length
    }

    pub fn refract_across(&self, normal: Vec3, etai_over_etat: f64) -> Self {
        // Snell stuff
        let cos_theta = normal.dot(-*self).min(1.0);
        let r_out_perp = (*self + normal * cos_theta) * etai_over_etat;
        let r_out_parallel = normal * -(1.0 - r_out_perp.length_squared()).abs().sqrt();
        r_out_perp + r_out_parallel
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}
