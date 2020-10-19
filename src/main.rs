extern crate image;
mod vec3;
use image::ImageBuffer;
use vec3::{Float, Vec3};

fn main() {
    let width = 200;
    let height = 100;
    let img = ImageBuffer::from_fn(width, height, |x, y| {
        let col = Vec3::new(
            (256 * x / width) as Float,
            (256 * y / height) as Float,
            256. * 0.2,
        );
        let r = col.x() as u8;
        let g = col.y() as u8;
        let b = col.z() as u8;

        image::Rgb([r, g, b])
    });
    img.save("test.png").unwrap();
}
