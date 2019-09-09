use crate::{bounding_box::BoundingBox, material::Material, ray::Ray, vec3::Vec3};

pub struct HitRecord<'a> {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f32, point: Vec3, normal: Vec3, material: &'a dyn Material) -> Self {
        HitRecord {
            t,
            point,
            normal,
            material,
        }
    }
}

pub trait Object {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;

    fn bounding_box(&self) -> Option<&BoundingBox>;
}

