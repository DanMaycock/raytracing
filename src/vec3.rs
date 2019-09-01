use std::ops::{Add, AddAssign, Sub};

#[derive(Default, Clone, Copy)]
pub struct Vec3 {
    data: [f32; 3],
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.data[0] + other.data[0], self.data[1] + other.data[1], self.data[2] + other.data[2])
    }
}

impl AddAssign for Vec3{
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.data[0] - other.data[0], self.data[1] - other.data[1], self.data[2] - other.data[2])
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 {
            data: [x, y, z]
        }
    }

    pub fn x(&self) -> f32 {
        self.data[0]
    }
    pub fn y(&self) -> f32 {
        self.data[1]
    }
    pub fn z(&self) -> f32 {
        self.data[2]
    }
    pub fn r(&self) -> f32 {
        self.data[0]
    }
    pub fn g(&self) -> f32 {
        self.data[1]
    }
    pub fn b(&self) -> f32 {
        self.data[2]
    }
    pub fn squared_length(&self) -> f32 {
        self.data[0] * self.data[0] + self.data[1] * self.data[1] + self.data[2] * self.data[2]
    }
    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }
    pub fn make_unit_vector(&mut self) {
        let k  = 1.0 / self.length();
        self.data[0] *= k;
        self.data[1] *= k;
        self.data[2] * k;
    }

    pub fn scalar_mul(&self, scalar: f32) -> Vec3 {
        Vec3::new(self.x() * scalar, self.y() * scalar, self.z() * scalar)
    }

    pub fn normalize(&self) -> Vec3 {
        self.scalar_mul(1.0 / self.length())
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn lerp(&self, other: &Vec3, t: f32) -> Vec3 {
        self.scalar_mul(1.0 - t) + other.scalar_mul(t)
    }
}