use crate::vec3::{Float, Vec3};

#[allow(dead_code)]
pub struct Ray {
    a: Vec3,
    b: Vec3,
}

#[allow(dead_code)]
impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Ray { a: a, b: b }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.a
    }

    pub fn direction(&self) -> &Vec3 {
        &self.b
    }

    pub fn point_at_parameter(&self, t: &Float) -> Vec3 {
        self.a + *t * self.b
    }
}
