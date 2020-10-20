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
        let target = hit_record.normal + random_unit_vector();

        let scattered = Ray::new(hit_record.point, target);
        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: Float,
}

impl Metal {
    pub fn new(albedo: &Vec3, fuzz: Float) -> Self {
        Self {
            albedo: *albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(&ray_in.direction().unit_vector(), &hit_record.normal);

        let scattered = Ray::new(
            hit_record.point,
            reflected + self.fuzz * random_in_uint_sphere(),
        );
        if scattered.direction().dot(&hit_record.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ir: Float,
}

impl Dielectric {
    pub fn new(ir: Float) -> Self {
        Dielectric { ir }
    }

    fn reflectance(cosine: Float, ref_idx: Float) -> Float {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let mut rng = rand::thread_rng();
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = ray_in.direction().unit_vector();

        let cos_theta = (-unit_direction).dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction;

        if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0, 1.0)
        {
            direction = reflect(&unit_direction, &hit_record.normal);
        } else {
            direction = refract(&unit_direction, &hit_record.normal, refraction_ratio);
        }

        Some((
            Vec3::new(1.0, 1.0, 1.0),
            Ray::new(hit_record.point, direction),
        ))
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

fn random_unit_vector() -> Vec3 {
    let mut rng = rand::thread_rng();
    let a = rng.gen_range(0.0, 2.0 * std::f32::consts::PI);
    let z: Float = rng.gen_range(-1.0, 1.0);
    let r = (1.0 - z * z).sqrt();
    Vec3::new(r * a.cos(), r * a.sin(), z)
}

pub fn reflect(vector: &Vec3, normal: &Vec3) -> Vec3 {
    vector - 2.0 * vector.dot(normal) * normal
}

/// 折射
pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: Float) -> Vec3 {
    let cos_theta = -uv.dot(n);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.squared_length()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}
