use crate::hittable::*;
use crate::ray::*;
use crate::vec3::*;
use rand::Rng;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)>;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: &Vec3) -> Self {
        Self { albedo: *albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = hit_record.point + hit_record.normal + random_in_uint_sphere();

        let scattered = Ray::new(hit_record.point, target - hit_record.point);
        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: &Vec3) -> Self {
        Self { albedo: *albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(&ray_in.direction().unit_vector(), &hit_record.normal);

        let scattered = Ray::new(hit_record.point, reflected);
        if scattered.direction().dot(&hit_record.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

/// 返回一个三维空间内的随机向量
/// 首先筛选在以原点为球心半径小于1的球内的向量
/// 这样能保证是均匀的分布
pub fn random_in_uint_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-1.0, 1.0),
        );
        if p.squared_length() <= 1.0 {
            return p;
        }
    }
}

pub fn reflect(vector: &Vec3, normal: &Vec3) -> Vec3 {
    vector - 2.0 * vector.dot(normal) * normal
}
