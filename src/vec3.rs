use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Debug, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }
    pub fn length(&self) -> f64 {
        self.norm().sqrt()
    }

    pub fn norm(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn normalize(self) -> Vec3 {
        let len = self.length();
        self / len
    }

    pub fn vec3_random_range(rng: &mut impl rand::Rng, range: std::ops::Range<f64>) -> Vec3 {
        Vec3::new(
            rng.gen_range(range.clone()),
            rng.gen_range(range.clone()),
            rng.gen_range(range),
        )
    }

    pub fn random_in_unit_sphere(rng: &mut impl rand::Rng) -> Vec3 {
        loop {
            let p = Vec3::vec3_random_range(rng, -1.0..1.0);
            if p.norm() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_in_hemisphere(&self, rng: &mut impl rand::Rng) -> Self {
        let in_unit_sphere = Vec3::random_in_unit_sphere(rng);
        if self.dot(&in_unit_sphere) > 0. {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn random_unit_vector(rng: &mut impl rand::Rng) -> Self {
        Self::random_in_unit_sphere(rng).normalize()
    }

    pub fn near_zero(&self) -> bool {
        const S: f64 = 1e-8;
        (self.x.abs() < S) && (self.y.abs() < S) && (self.z.abs() < S)
    }
}

impl AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl SubAssign<&Vec3> for Vec3 {
    fn sub_assign(&mut self, other: &Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        self.mul_assign(1. / other)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for &Vec3 {
    type Output = Vec3;
    fn add(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Self) -> Self::Output {
        &self + &other
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Self::Output {
        self + &other
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, other: &Vec3) -> Self::Output {
        &self + other
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;
    fn sub(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Self::Output {
        self - &other
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Self) -> Self::Output {
        &self - &other
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: &Vec3) -> Self::Output {
        &self - other
    }
}

impl Mul for &Vec3 {
    type Output = Vec3;
    fn mul(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Self) -> Self::Output {
        &self * &other
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other: &Vec3) -> Self::Output {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Self::Output {
        self * &other
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, other: f64) -> Self::Output {
        other * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f64) -> Self::Output {
        (1. / other) * &self
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, other: f64) -> Self::Output {
        (1. / other) * self
    }
}
