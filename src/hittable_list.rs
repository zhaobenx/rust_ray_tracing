use crate::hittable::*;
use crate::ray::Ray;
use crate::vec3::*;

// struct HittableList {
//     list: Vec<dyn Hittable>,
// }

pub fn hit(list: &Vec<Box<dyn Hittable>>, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
    let mut closest_so_far = t_max;
    let mut hit_record = None;
    for i in list.iter() {
        hit_record = match i.hit(ray, t_min, closest_so_far) {
            Some(hit_record) => {
                closest_so_far = hit_record.t;
                Some(hit_record)
            }
            None => hit_record,
        }
    }
    hit_record
}
