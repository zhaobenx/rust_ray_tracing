extern crate image;

use image::ImageBuffer;
use std::ops;

type Float = f32;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Vec3 {
    x: Float,
    y: Float,
    z: Float,
}

impl Vec3 {
    fn new(x: Float, y: Float, z: Float) -> Self {
        Vec3 { x: x, y: y, z: z }
    }
    fn x(&self) -> Float {
        self.x
    }

    fn y(&self) -> Float {
        self.y
    }

    fn z(&self) -> Float {
        self.z
    }

    fn length(&self) -> Float {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn squared_length(&self) -> Float {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn normalized(&mut self) {
        let length = self.length();
        *self = *self / length;
    }

    fn dot(&self, other: &Self) -> Float {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross(&self, other: &Self) -> Self {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Self) -> Self {
        Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl ops::Mul<Float> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Float) -> Self {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Div<Float> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Float) -> Self {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl ops::Mul<Vec3> for Float {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

#[cfg(not(test))]
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

// 当且仅当测试套件运行时，才条件编译 `test` 模块。
#[cfg(test)]
mod test {
    use crate::Vec3;
    #[test]
    fn test_add() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(1.0, 2.0, 3.0);
        assert!(a + b == Vec3::new(2., 4., 6.));
    }
    #[test]
    fn test_neg() {
        let a = Vec3::new(1.0, -2.0, 3.0);
        assert!(-a == Vec3::new(-1.0, 2.0, -3.0));
    }
    #[test]
    fn test_mul() {
        let a = Vec3::new(1.0, -2.0, 3.0);
        let b = Vec3::new(3.0, -4.0, 5.0);
        let c = 3.0;
        assert!(a * b == Vec3::new(3.0, 8.0, 15.0));
        assert!(a * c == Vec3::new(3.0, -6.0, 9.0));
        assert!(c * a == Vec3::new(3.0, -6.0, 9.0));
    }
    #[test]
    fn test_div() {
        let a = Vec3::new(6.0, -20.0, 120.0);
        let b = Vec3::new(3.0, -4.0, 5.0);
        let c = 2.0;
        assert!(a / b == Vec3::new(2.0, 5.0, 24.0));
        assert!(a / c == Vec3::new(3.0, -10.0, 60.0));
    }

    #[test]
    fn test_length() {
        let a = Vec3::new(3.0, 4.0, 0.0);
        let b = Vec3::new(3.0, -4.0, 12.0);

        assert!(a.length() == 5.0);
        assert!(b.length() == 13.0);
    }
    #[test]
    fn test_squared_length() {
        let a = Vec3::new(3.0, 4.0, 0.0);
        let b = Vec3::new(3.0, -4.0, 12.0);
        assert!(a.squared_length() == 25.0);
        assert!(b.squared_length() == 169.0);
    }
    #[test]
    fn test_normalized() {
        let mut a = Vec3::new(4.0, 0.0, 0.0);
        a.normalized();
        let mut b = Vec3::new(4.0, 12.0, 3.0);
        b.normalized();
        assert!(a == Vec3::new(1.0, 0.0, 0.0));
        assert!(a.length() == 1.0);
        assert!(b == Vec3::new(4.0 / 13.0, 12.0 / 13.0, 3.0 / 13.0));
        assert!(b.length() == 1.0);
    }
    #[test]
    fn test_dot() {
        let a = Vec3::new(3.0, 4.0, 0.0);
        let b = Vec3::new(3.0, -4.0, 12.0);
        assert!(a.dot(&b) == -7.0);
    }

    #[test]
    fn test_cross() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(1.0, 5.0, 7.0);
        assert!(a.cross(&b) == Vec3::new(-1.0, -4.0, 3.0));
        let a = Vec3::new(-1.0, -2.0, 3.0);
        let b = Vec3::new(4.0, 0.0, -8.0);
        assert!(a.cross(&b) == Vec3::new(16.0, 4.0, 8.0));
    }
}
