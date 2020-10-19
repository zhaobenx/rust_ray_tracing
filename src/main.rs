extern crate image;
mod ray;
mod vec3;
use image::ImageBuffer;
use ray::Ray;
use vec3::{Float, Vec3};

fn color(ray: &Ray) -> Vec3 {
    let unit_direction = ray.direction().unit_vector();
    let t: Float = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let width = 200;
    let height = 100;
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let img = ImageBuffer::from_fn(width, height, |x, y| {
        let u = x as Float / width as Float;
        let v = y as Float / height as Float;
        let ray = Ray::new(origin, lower_left_corner + u*horizontal+v*vertical);
        let col = color(&ray);
        let r = (256.0 * col.x()) as u8;
        let g = (256.0 * col.y()) as u8;
        let b = (256.0 * col.z()) as u8;

        image::Rgb([r, g, b])
    });
    img.save("test.png").unwrap();
}
