use crate::vec3::Vec3;

#[derive(Clone)]
pub struct Ray {
    origin: Vec3,
    dir: Vec3,
    time: f32,
}

impl Ray {

    pub fn new(origin: Vec3, dir: Vec3) -> Self {
        Ray {origin, dir, time: 0.0}
    }

    pub fn new_at_time(origin: Vec3, dir: Vec3, time: f32) -> Self {
        Ray {origin, dir, time}
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn time(&self) -> f32 {
        self.time
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin + self.dir.scalar_mul(t)
    } 
} 