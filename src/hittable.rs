use crate::ray::Ray;
use crate::vec3::{Float, Vec3};

pub struct HitRecord {
    /// 摄像机向量到交汇点的距离（长度的倍数）
    pub t: Float,
    /// 交汇点
    pub point: Vec3,
    /// 法向量
    pub normal: Vec3,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            t: 0.0,
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Vec3,
    radius: Float,
}

impl Sphere {
    pub fn new(center: Vec3, radius: Float) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    /// 注意：这里作者把delta的系数进行了约分，所以求根公式里的系数消掉了
    /// 同时有两个求解的过程，分别对应着两个根，即和球的两个交点
    ///
    /// 检测射线是否和球体有交汇
    /// 返回值为射线方向和球体的交点的长度是射线的`t`倍
    /// 如果不相交则返回-1，相交返回`t`
    ///
    /// `oc` 为球心指向射线原点的射线
    /// `|oc - t*ray.direction()|^2 == r^2`
    /// 展开即得关于t的二次方程，解之即得下面的abc
    ///
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let mut hit_record = HitRecord::new();
        let oc = ray.origin() - &self.center;
        let a = ray.direction().dot(ray.direction());
        let b = oc.dot(ray.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let root1 = (-b - (b * b - a * c).sqrt()) / a;
            if root1 < t_max && root1 > t_min {
                hit_record.t = root1;
                hit_record.point = ray.point_at_parameter(&hit_record.t);
                hit_record.normal = (hit_record.point - self.center) / self.radius;
                return Some(hit_record);
            }
            let root2 = (-b + (b * b - a * c).sqrt()) / a;
            if root2 < t_max && root2 > t_min {
                hit_record.t = root2;
                hit_record.point = ray.point_at_parameter(&hit_record.t);
                hit_record.normal = (hit_record.point - self.center) / self.radius;
                return Some(hit_record);
            }
        }
        None
    }
}
