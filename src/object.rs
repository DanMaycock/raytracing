use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

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
}

#[derive(Default)]
pub struct ObjectList {
    list: Vec<Box<dyn Object>>,
}

impl Object for ObjectList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_hit = t_max;
        for object in self.list.iter() {
            if let Some(new_hit_record) = (*object).hit(ray, t_min, closest_hit) {
                closest_hit = new_hit_record.t;
                hit_record = Some(new_hit_record);
            }
        }
        hit_record
    }
}

impl ObjectList {
    pub fn push(&mut self, object: Box<dyn Object>) {
        self.list.push(object);
    }
}
