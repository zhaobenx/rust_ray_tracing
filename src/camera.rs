use crate::ray::*;
use crate::vec3::*;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: &Vec3,
        lookat: &Vec3,
        vup: &Vec3,
        vfov: Float,
        aspect_ratio: Float,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin = *lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Camera {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
        }
    }

    pub fn get_ray(&self, s: Float, t: Float) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s* self.horizontal + t * self.vertical - self.origin,
        )
    }
}
