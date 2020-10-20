extern crate image;
mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod vec3;

use camera::*;
use hittable::*;
use hittable_list::hit;
use image::ImageBuffer;
use material::*;
use rand::Rng;
use ray::Ray;
use std::rc::Rc;
use std::time::Instant;
use vec3::{Float, Vec3};

fn ray_color(ray: &Ray, world: &Vec<Box<dyn Hittable>>, depth: i32) -> Vec3 {
    if depth < 0 {
        return Vec3::zero();
    }

    match hit(world, ray, 0.001, Float::MAX) {
        Some(hit_record) => match hit_record.material.scatter(ray, &hit_record) {
            Some((attenuation, scattered)) => {
                attenuation * ray_color(&scattered, &world, depth - 1)
            }
            None => Vec3::zero(),
        },
        None => {
            let t = 0.5 * (ray.direction().unit_vector().y() + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    println!("Start running...");
    let start = Instant::now();
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as Float / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let mut rng = rand::thread_rng();
    let camera = Camera::new();
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

    let material_ground = Rc::new(Lambertian::new(&Vec3::new(0.8, 0.8, 0.0)));
    // let material_center = Rc::new(Dielectric::new(1.5));
    // let material_left = Rc::new(Dielectric::new(1.5));
    let material_center = Rc::new(Lambertian::new(&Vec3::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::new(&Vec3::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Rc::new(Metal::new(&Vec3::new(0.8, 0.6, 0.2), 1.0));

    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let img = ImageBuffer::from_fn(width, height, |x, y| {
        let mut pixel_color = Vec3::zero();
        for _ in 0..samples_per_pixel {
            let u = (x as Float + rng.gen_range(0.0, 1.0)) / (width - 1) as Float;
            let v = 1.0 - (y as Float + rng.gen_range(0.0, 1.0)) / (height - 1) as Float;
            let ray = camera.get_ray(u, v);
            pixel_color += ray_color(&ray, &world, max_depth);
        }

        // Divide the color by the number of samples and gamma-correct for gamma=2.0.
        let scale = 1.0 / samples_per_pixel as Float;
        let r = pixel_color.x();
        let g = pixel_color.y();
        let b = pixel_color.z();

        let r = (scale * r).sqrt();
        let g = (scale * g).sqrt();
        let b = (scale * b).sqrt();

        let r = (255.0 * r) as u8;
        let g = (255.0 * g) as u8;
        let b = (255.0 * b) as u8;

        image::Rgb([r, g, b])
    });
    let elapsed = start.elapsed();
    img.save("chapter10.4.png").unwrap();
    println!("Time spent: {} ms", elapsed.as_millis());
}
