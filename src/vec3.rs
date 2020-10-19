use std::ops;
pub type Float = f32;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    x: Float,
    y: Float,
    z: Float,
}

impl Vec3 {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn zero() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn x(&self) -> Float {
        self.x
    }

    pub fn y(&self) -> Float {
        self.y
    }

    pub fn z(&self) -> Float {
        self.z
    }

    pub fn length(&self) -> Float {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn squared_length(&self) -> Float {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalized(&mut self) {
        let length = self.length();
        *self = *self / length;
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub fn dot(&self, other: &Self) -> Float {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
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

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
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

impl ops::DivAssign<Float> for Vec3 {
    fn div_assign(&mut self, rhs: Float) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
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
