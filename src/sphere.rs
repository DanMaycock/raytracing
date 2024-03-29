use crate::object::{Object, HitRecord};
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;
use crate::bounding_box::BoundingBox;

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Box<dyn Material>,
    bbox: BoundingBox,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Box<dyn Material>) -> Self {
        let bbox = BoundingBox::new(
            center - Vec3::new(radius, radius, radius), 
            center + Vec3::new(radius, radius, radius)
        );
        Sphere {center, radius, material, bbox}
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
                return Some(HitRecord::new(t, point, normal, self.material.as_ref()))
            }
            let t = (-b + discriminant.sqrt()) / (2.0 * a);
            if t < t_max && t > t_min {
                let point = ray.point_at_parameter(t);
                let normal = (point - self.center).scalar_mul(1.0 / self.radius);
                return Some(HitRecord::new(t, point, normal, self.material.as_ref()))
            } 
        }
        None
    }

    fn bounding_box(&self) -> Option<&BoundingBox> {
        Some(&self.bbox)
    }
}

pub struct MovingSphere {
    start_center: Vec3,
    end_center: Vec3,
    radius: f32,
    start_time: f32,
    end_time: f32,
    material: Box<dyn Material>,
    bbox: BoundingBox,
}


impl MovingSphere {
    pub fn new(start_center: Vec3, end_center: Vec3, radius: f32, start_time: f32, end_time: f32, material: Box<dyn Material>) -> Self {
        let bbox = BoundingBox::containing_box(
            &BoundingBox::new(
                start_center - Vec3::new(radius, radius, radius), 
                start_center + Vec3::new(radius, radius, radius)
            ),
            &BoundingBox::new(
                end_center - Vec3::new(radius, radius, radius), 
                end_center + Vec3::new(radius, radius, radius)
            ),
        );
        MovingSphere {start_center, end_center, radius, start_time, end_time, material, bbox}
    }

    #[inline]
    fn center(&self, time: f32) -> Vec3 {
        self.start_center + (self.end_center - self.start_center).scalar_mul( (time - self.start_time) / (self.end_time - self.start_time))
    }
}

impl Object for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center(ray.time());
        let a = ray.direction().dot(&ray.direction());
        let b = 2.0 * oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            if t < t_max && t > t_min {
                let point = ray.point_at_parameter(t);
                let normal = (point - self.center(ray.time())).scalar_mul(1.0 / self.radius);
                return Some(HitRecord::new(t, point, normal, self.material.as_ref()))
            }
            let t = (-b + discriminant.sqrt()) / (2.0 * a);
            if t < t_max && t > t_min {
                let point = ray.point_at_parameter(t);
                let normal = (point - self.center(ray.time())).scalar_mul(1.0 / self.radius);
                return Some(HitRecord::new(t, point, normal, self.material.as_ref()))
            } 
        }
        None
    }

    fn bounding_box(&self) -> Option<&BoundingBox> {
        Some(&self.bbox)
    }
}