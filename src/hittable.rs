use crate::material::*;
use crate::ray::Ray;
use crate::vec3::{Float, Vec3};
use std::rc::Rc;

pub struct HitRecord {
    /// 摄像机向量到交汇点的距离（长度的倍数）
    pub t: Float,
    /// 交汇点
    pub point: Vec3,
    /// 法向量
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub front_face: bool,
}

impl HitRecord {
    // pub fn new(t: Float, point: Vec3, normal: Vec3, material: Rc<dyn Material>) -> Self {
    //     HitRecord {
    //         t,
    //         point: point,
    //         normal: normal,
    //         material,
    //         front_face: true,
    //     }
    // }

    pub fn new(
        t: Float,
        point: Vec3,
        outward_normal: Vec3,
        material: Rc<dyn Material>,
        ray: &Ray,
    ) -> Self {
        let front_face = ray.direction().dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord {
            t,
            point,
            normal,
            material,
            front_face,
        }
    }

    // /// 设置表面是否为前面，通过视线和法向的夹角来确定
    // /// 保证这里的normal一定是和视线夹角大于180°
    // pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
    //     self.front_face = ray.direction().dot(outward_normal) < 0.0;
    //     self.normal = if self.front_face {
    //         *outward_normal
    //     } else {
    //         -*outward_normal
    //     }
    // }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Vec3,
    radius: Float,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: Float, material: Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
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
        let oc = ray.origin() - &self.center;
        let a = ray.direction().dot(ray.direction());
        let b = oc.dot(ray.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let root1 = (-b - (b * b - a * c).sqrt()) / a;
            if root1 < t_max && root1 > t_min {
                let point = ray.point_at_parameter(&root1);
                let hit_record = HitRecord::new(
                    root1,
                    point,
                    (point - self.center) / self.radius,
                    Rc::clone(&self.material),
                    ray,
                );
                return Some(hit_record);
            }
            let root2 = (-b + (b * b - a * c).sqrt()) / a;
            if root2 < t_max && root2 > t_min {
                let point = ray.point_at_parameter(&root2);
                let hit_record = HitRecord::new(
                    root2,
                    point,
                    (point - self.center) / self.radius,
                    Rc::clone(&self.material),
                    ray,
                );
                return Some(hit_record);
            }
        }
        None
    }
}
