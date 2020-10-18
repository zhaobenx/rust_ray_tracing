extern crate image;

use image::ImageBuffer;

fn main() {
    let width = 200;
    let height = 100;
    let img = ImageBuffer::from_fn(width, height, |x, y| {
        let r = (256 * x / width) as u8;
        let g = (256 * y / height) as u8;
        let b = (256. * 0.2) as u8;

        image::Rgb([r, g, b])
    });
    img.save("test.png").unwrap();
}
