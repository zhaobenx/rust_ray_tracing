extern crate image;
mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod vec3;
use camera::*;
use hittable::*;
use hittable_list::hit;
use image::ImageBuffer;
use rand::Rng;
use ray::Ray;
use std::time::Instant;
use vec3::{Float, Vec3};
fn color(ray: &Ray, world: &Vec<Box<dyn Hittable>>) -> Vec3 {
    match hit(world, ray, 0.0, Float::MAX) {
        Some(hit_record) => {
            0.5 * Vec3::new(
                hit_record.normal.x() + 1.0,
                hit_record.normal.y() + 1.0,
                hit_record.normal.z() + 1.0,
            )
        }
        None => {
            let t = 0.5 * (ray.direction().unit_vector().y() + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    let start = Instant::now();
    let width = 200;
    let height = 100;
    let ns = 100;
    let mut rng = rand::thread_rng();
    let camera = Camera::new();
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();
    world.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let img = ImageBuffer::from_fn(width, height, |x, y| {
        let mut col = Vec3::zero();
        for _ in 0..ns {
            let u = (x as Float + rng.gen_range(0.0, 1.0)) / width as Float;
            let v = 1.0 - (y as Float + rng.gen_range(0.0, 1.0)) / height as Float;
            let ray = camera.get_ray(u, v);
            col += color(&ray, &world);
        }
        col /= ns as Float;
        let r = (256.0 * col.x()) as u8;
        let g = (256.0 * col.y()) as u8;
        let b = (256.0 * col.z()) as u8;

        image::Rgb([r, g, b])
    });
    let elapsed = start.elapsed();
    img.save("chapter6.png").unwrap();
    println!("Time spent: {} ms", elapsed.as_millis());
}
