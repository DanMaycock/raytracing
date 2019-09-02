use crate::ray::Ray;
use crate::utils::random_in_unit_disk;
use crate::vec3::Vec3;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
    open_time: f32,
    close_time: f32,
}

impl Camera {
    // v_fov is top to bottom in radians
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        v_up: Vec3,
        v_fov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
        open_time: f32,
        close_time: f32,
    ) -> Self {
        let lens_radius = aperture / 2.0;
        let half_height = (v_fov / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (look_from - look_at).normalize();
        let u = (v_up.cross(&w)).normalize();
        let v = w.cross(&u);

        Camera {
            lower_left_corner: look_from
                - u.scalar_mul(focus_dist * half_width)
                - v.scalar_mul(focus_dist * half_height)
                - w.scalar_mul(focus_dist),
            horizontal: u.scalar_mul(2.0 * focus_dist * half_width),
            vertical: v.scalar_mul(2.0 * focus_dist * half_height),
            origin: look_from,
            u,
            v,
            w,
            lens_radius,
            open_time,
            close_time,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = random_in_unit_disk().scalar_mul(self.lens_radius);
        let offset = self.u.scalar_mul(rd.x()) + self.v.scalar_mul(rd.y());
        let time = self.open_time + thread_rng().gen::<f32>() * (self.close_time - self.open_time);
        Ray::new_at_time(
            self.origin + offset,
            self.lower_left_corner + self.horizontal.scalar_mul(s) + self.vertical.scalar_mul(t)
                - self.origin
                - offset,
            time,
        )
    }
}
