use crate::{
    bounding_box::BoundingBox,
    object::{HitRecord, Object},
    ray::Ray,
};

use rand::{thread_rng, Rng};

pub struct BvhNode {
    left: Box<dyn Object>,
    right: Box<dyn Object>,
    bbox: BoundingBox,
}

impl Object for BvhNode {
    fn bounding_box(&self) -> Option<&BoundingBox> {
        Some(&self.bbox)
    }

    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if self.bbox.hit(ray, t_min, t_max) {
            let left_rec = self.left.hit(ray, t_min, t_max);
            let right_rec = self.right.hit(ray, t_min, t_max);
            if left_rec.is_some() && right_rec.is_some() {
                if left_rec.as_ref().unwrap().t < right_rec.as_ref().unwrap().t {
                    left_rec
                } else {
                    right_rec
                }
            } else if left_rec.is_some() {
                left_rec
            } else if right_rec.is_some() {
                right_rec
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl BvhNode {
    pub fn new(mut objects: Vec<Box<dyn Object>>) -> Self {
        let axis_index = thread_rng().gen_range(0, 3);
        objects.sort_unstable_by(|a, b| {
            a.bounding_box().unwrap().min.data()[axis_index]
                .partial_cmp(&b.bounding_box().unwrap().min.data()[axis_index])
                .unwrap()
        });
        let (left, right): (Box<dyn Object>, Box<dyn Object>) = if objects.len() == 1 {
            panic!("Cannot split a vector of length 1 into two");
        } else if objects.len() == 2 {
            let right = objects.pop().unwrap();
            let left = objects.pop().unwrap();
            (left, right)
        } else if objects.len() == 3 {
            let right = objects.pop().unwrap();
            (Box::new(BvhNode::new(objects)), right)
        } else {
            let right_half = objects.split_off(objects.len() / 2);
            (
                Box::new(BvhNode::new(objects)),
                Box::new(BvhNode::new(right_half)),
            )
        };

        let bbox = BoundingBox::containing_box(
            &left.bounding_box().unwrap(),
            &right.bounding_box().unwrap(),
        );

        BvhNode { left, right, bbox }
    }
}
