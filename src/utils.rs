use crate::vec3::Vec3;
use rand::{prelude::thread_rng, Rng};

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = thread_rng();
    loop {
        let p =
            Vec3::new(rng.gen(), rng.gen(), rng.gen()).scalar_mul(2.0) - Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            return p
        }
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = thread_rng();
    loop {
        let p = Vec3::new(rng.gen(), rng.gen(), 0.0).scalar_mul(2.0) - Vec3::new(1.0, 1.0, 0.0);
        if p.dot(&p) < 1.0 {
            return p
        }
    }
}

pub fn reflect(vector: &Vec3, normal: &Vec3) -> Vec3 {
    *vector - normal.scalar_mul(2.0 * vector.dot(normal))
}

pub fn refract(vector: &Vec3, normal: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let unit_vector = vector.normalize();
    let dot_product = unit_vector.dot(normal);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dot_product * dot_product);
    if discriminant > 0.0 {
        Some(
            (unit_vector - normal.scalar_mul(dot_product)).scalar_mul(ni_over_nt)
                - normal.scalar_mul(discriminant.sqrt()),
        )
    } else {
        None
    }
}

pub fn schlick(cosine: f32, refractive_index: f32) -> f32 {
    let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
