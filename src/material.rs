use crate::{
    object::HitRecord,
    ray::Ray,
    utils::{random_in_unit_sphere, reflect, refract, schlick},
    vec3::Vec3,
};

use rand::Rng;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)>;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = hit_record.point + hit_record.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit_record.point, target - hit_record.point);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(&ray_in.direction().normalize(), &hit_record.normal);
        let scattered = Ray::new(
            hit_record.point,
            reflected + random_in_unit_sphere().scalar_mul(self.fuzz),
        );
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        let fuzz = fuzz.min(1.0).max(0.0);
        Metal { albedo, fuzz }
    }
}

pub struct Dielectric {
    refractive_index: f32,
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let (outward_normal, ni_over_nt, cosine) =
            if ray_in.direction().dot(&hit_record.normal) > 0.0 {
                (
                    -hit_record.normal,
                    self.refractive_index,
                    self.refractive_index * ray_in.direction().dot(&hit_record.normal)
                        / ray_in.direction().length(),
                )
            } else {
                (
                    hit_record.normal,
                    1.0 / self.refractive_index,
                    -ray_in.direction().dot(&hit_record.normal) / ray_in.direction().length(),
                )
            };

        let reflected = reflect(&ray_in.direction(), &hit_record.normal);
        let refracted = refract(&ray_in.direction(), &outward_normal, ni_over_nt);
        let reflect_prob = if refracted.is_some() {
            schlick(cosine, self.refractive_index)
        } else {
            1.0
        };

        if rand::thread_rng().gen::<f32>() < reflect_prob {
            Some((attenuation, Ray::new(hit_record.point, reflected)))
        } else {
            Some((attenuation, Ray::new(hit_record.point, refracted.unwrap())))
        }
    }
}

impl Dielectric {
    pub fn new(refractive_index: f32) -> Self {
        Dielectric { refractive_index }
    }
}
