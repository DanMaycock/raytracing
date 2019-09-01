use crate::vec3::Vec3;

#[derive(Clone)]
pub struct Ray {
    origin: Vec3,
    dir: Vec3,
}

impl Ray {

    pub fn new(origin: Vec3, dir: Vec3) -> Self {
        Ray {origin, dir}
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin + self.dir.scalar_mul(t)
    } 
} 