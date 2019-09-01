#![warn(clippy::all)]
mod camera;
mod material;
mod object;
mod ray;
mod sphere;
mod utils;
mod vec3;

use rand::prelude::*;
use std::fs::File;
use std::io::{Error, Write};

use crate::camera::Camera;
use crate::material::{Lambertian, Metal, Dielectric};
use crate::object::{Object, ObjectList};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

fn get_color(ray: &Ray, world: &dyn Object, depth: u32) -> Vec3 {
    if let Some(hit_record) = world.hit(ray, 0.001, std::f32::MAX) {
        if depth < 50 {
            if let Some((attenuation, scattered)) = hit_record.material.scatter(ray, &hit_record) {
                return attenuation * get_color(&scattered, world, depth + 1);
            }
        }
        Vec3::new(0.0, 0.0, 0.0)
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
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Box::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0)),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Box::new(Dielectric::new(1.5)),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.45,
        Box::new(Dielectric::new(1.5)),
    )));

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let v = (j as f32 + rng.gen::<f32>()) / ny as f32;
                let ray = camera.get_ray(u, v);
                color += get_color(&ray, &world, 0);
            }
            color = color.scalar_mul(1.0 / ns as f32);
            color = Vec3::new(color.x().sqrt(), color.y().sqrt(), color.z().sqrt());
            let ir = (255.99 * color.r()) as i32;
            let ig = (255.99 * color.g()) as i32;
            let ib = (255.99 * color.b()) as i32;
            writeln!(output, "{} {} {}", ir, ig, ib)?;
        }
    }

    Ok(())
}
