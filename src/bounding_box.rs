use crate::{ray::Ray, vec3::Vec3};

#[derive(Clone, Debug)]
pub struct BoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

impl BoundingBox {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        BoundingBox { min, max }
    }

    pub fn containing_box(box0: &BoundingBox, box1: &BoundingBox) -> BoundingBox {
        let min = Vec3::new(
            box0.min.x().min(box1.min.x()),
            box0.min.y().min(box1.min.y()),
            box0.min.z().min(box1.min.z()),
        );
        let max = Vec3::new(
            box0.max.x().max(box1.max.x()),
            box0.max.y().max(box1.max.y()),
            box0.max.z().max(box1.max.z()),
        );
        BoundingBox::new(min, max)
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        for axis_index in 0..3 {
            let inv_dir: f32 = 1.0 / ray.direction().data()[axis_index];
            let mut t0 = (self.min.data()[axis_index] - ray.origin().data()[axis_index]) * inv_dir;
            let mut t1 = (self.max.data()[axis_index] - ray.origin().data()[axis_index]) * inv_dir;
            if inv_dir < 0.0 {
                std::mem::swap(&mut t0, &mut t1)
            }
            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}
