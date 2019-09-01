#![warn(clippy::all)]
mod ray;
mod vec3;
mod object;
mod sphere;
mod camera;

use std::fs::File;
use std::io::{Error, Write};
use rand::prelude::*;

use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::object::{Object, ObjectList};
use crate::sphere::Sphere;
use crate::camera::Camera;

fn get_color(ray: &Ray, world: &dyn Object) -> Vec3 {
    if let Some(hit_record) = world.hit(ray, 0.0, std::f32::MAX) {
        (hit_record.normal + Vec3::new(1.0, 1.0, 1.0)).scalar_mul(0.5)
    } else {
        let unit_direction = ray.direction().normalize();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Vec3::new(1.0, 1.0, 1.0).lerp(&Vec3::new(0.5, 0.7, 1.0), t)
    }
}

fn main() -> Result<(), Error> {
    let path = "output.ppm";
    let mut output = File::create(path)?;
    let nx = 200;
    let ny = 100;
    let ns = 100;
    write!(output, "P3\n{} {} \n255\n", nx, ny)?;

    let camera = Camera::default();
    let mut rng = thread_rng();

    let mut world = ObjectList::default();
    world.push(Box::new(Sphere::new(Vec3::new(0.0,0.0,-1.0), 0.5)));
    world.push(Box::new(Sphere::new(Vec3::new(0.0,-100.5,-1.0), 100.0)));

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let v = (j as f32 + rng.gen::<f32>()) / ny as f32;
                let ray = camera.get_ray(u, v);
                color += get_color(&ray, &world);
            }
            color = color.scalar_mul(1.0 / ns as f32);
            let ir = (255.99 * color.r()) as i32;
            let ig = (255.99 * color.g()) as i32;
            let ib = (255.99 * color.b()) as i32;
            writeln!(output, "{} {} {}", ir, ig, ib)?;
        }
    }

    Ok(())
}
