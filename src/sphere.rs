use crate::object::{Object, HitRecord};
use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere {center, radius}
    }
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(&ray.direction());
        let b = 2.0 * oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            if t < t_max && t > t_min {
                let point = ray.point_at_parameter(t);
                let normal = (point - self.center).scalar_mul(1.0 / self.radius);
                return Some(HitRecord::new(t, point, normal))
            }
            let t = (-b + discriminant.sqrt()) / (2.0 * a);
            if t < t_max && t > t_min {
                let point = ray.point_at_parameter(t);
                let normal = (point - self.center).scalar_mul(1.0 / self.radius);
                return Some(HitRecord::new(t, point, normal))
            } 
        }
        None
    }
}