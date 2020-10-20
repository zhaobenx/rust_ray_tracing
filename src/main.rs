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
    let aspect_ratio = 3.0 / 2.0;
    let width = 1200;
    let height = (width as Float / aspect_ratio) as u32;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let mut rng = rand::thread_rng();
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        &lookfrom,
        &lookat,
        &vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let world = random_scene();

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
    img.save("final.png").unwrap();
    println!("Time spent: {} ms", elapsed.as_millis());
}

fn random_scene() -> Vec<Box<dyn Hittable>> {
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();
    let mut rng = rand::thread_rng();
    let material_ground = Rc::new(Lambertian::new(&Vec3::new(0.5, 0.5, 0.5)));
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(0.0, 1.0);
            let center = Vec3::new(
                a as Float + 0.9 * rng.gen_range(0.0, 1.0),
                0.2,
                b as Float + 0.9 * rng.gen_range(0.0, 1.0),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random() * Vec3::random();
                    let sphere_material = Rc::new(Lambertian::new(&albedo));

                    world.push(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::new(
                        rng.gen_range(0.5, 1.0),
                        rng.gen_range(0.5, 1.0),
                        rng.gen_range(0.5, 1.0),
                    );
                    let fuzz = rng.gen_range(0.0, 0.5);
                    let sphere_material = Rc::new(Metal::new(&albedo, fuzz));

                    world.push(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    world.push(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }
    let material1 = Rc::new(Dielectric::new(1.5));
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Rc::new(Lambertian::new(&Vec3::new(0.4, 0.2, 0.1)));
    world.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Rc::new(Metal::new(&Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}
