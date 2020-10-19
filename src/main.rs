extern crate image;
mod ray;
mod vec3;
use image::ImageBuffer;
use ray::Ray;
use std::time::Instant;
use vec3::{Float, Vec3};

/// 检测射线是否和球体有交汇
/// 返回值为射线方向和球体的交点的长度是射线的`t`倍
/// 如果不相交则返回-1，相交返回`t`
/// 
/// `oc` 为球心指向射线原点的射线
/// `|oc - t*ray.direction()|^2 == r^2` 
/// 展开即得关于t的二次方程，解之即得下面的abc
/// 
fn hit_sphere(center: &Vec3, radius: &Float, ray: &Ray) -> Float {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * oc.dot(ray.direction());
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        -b - discriminant.sqrt() / (2.0 * a)
    }
}

fn color(ray: &Ray) -> Vec3 {
    let t = hit_sphere(&Vec3::new(0.0, 0.0, -1.0), &0.5, ray);
    if t > 0.0 {
        let n = ray.point_at_parameter(&t).unit_vector() - Vec3::new(0.0, 0.0, -1.0);
        return 0.5 * Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let start = Instant::now();
    let width = 200;
    let height = 100;
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let img = ImageBuffer::from_fn(width, height, |x, y| {
        let u = x as Float / width as Float;
        let v = y as Float / height as Float;
        let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
        let col = color(&ray);
        let r = (256.0 * col.x()) as u8;
        let g = (256.0 * col.y()) as u8;
        let b = (256.0 * col.z()) as u8;

        image::Rgb([r, g, b])
    });
    let elapsed = start.elapsed();
    img.save("chapter4.1.png").unwrap();
    println!("Time spent: {} ms", elapsed.as_millis());
}
