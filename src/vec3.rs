use std::ops::{Add, AddAssign, Sub, Mul, Neg};
use std::iter::Sum;

#[derive(Default, Clone, Copy, Debug)]
pub struct Vec3 {
    data: [f32; 3],
}

impl Add for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.data[0] + other.data[0], self.data[1] + other.data[1], self.data[2] + other.data[2])
    }
}

impl AddAssign for Vec3{
    #[inline]
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.data[0] - other.data[0], self.data[1] - other.data[1], self.data[2] - other.data[2])
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self.data[0] * other.data[0],self.data[1] * other.data[1],self.data[2] * other.data[2])
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    #[inline]
    fn neg(self) -> Vec3 {
        Vec3::new(-self.data[0], -self.data[1], -self.data[2])
    }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item=Vec3>>(iter: I) -> Vec3 {
        iter.fold(Vec3::new(0.0, 0.0, 0.0), |a, b| a + b)
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 {
            data: [x, y, z]
        }
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self.data[0]
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.data[1]
    }

    #[inline]
    pub fn z(&self) -> f32 {
        self.data[2]
    }

    #[inline]
    pub fn r(&self) -> f32 {
        self.data[0]
    }

    #[inline]
    pub fn g(&self) -> f32 {
        self.data[1]
    }

    #[inline]
    pub fn b(&self) -> f32 {
        self.data[2]
    }

    #[inline]
    pub fn squared_length(&self) -> f32 {
        self.data[0] * self.data[0] + self.data[1] * self.data[1] + self.data[2] * self.data[2]
    }

    #[inline]
    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    #[inline]
    pub fn scalar_mul(&self, scalar: f32) -> Vec3 {
        Vec3::new(self.x() * scalar, self.y() * scalar, self.z() * scalar)
    }

    #[inline]
    pub fn normalize(&self) -> Vec3 {
        self.scalar_mul(1.0 / self.length())
    }

    #[inline]
    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    #[inline]
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.data[1] * other.data[2] - self.data[2] * other.data[1],
            self.data[2] * other.data[0] - self.data[0] * other.data[2],
            self.data[0] * other.data[1] - self.data[1] * other.data[0],
        )
    }

    #[inline]
    pub fn lerp(&self, other: &Vec3, t: f32) -> Vec3 {
        self.scalar_mul(1.0 - t) + other.scalar_mul(t)
    }

    pub fn data(&self) -> &[f32] {
        &self.data
    }
}