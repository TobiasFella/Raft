pub(crate) use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::{Add, AddAssign, Sub, SubAssign};

const FLOAT_MIN_DIFF: f32 = 0.001;

#[derive(Debug, Clone, Copy)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn len(&self) -> f32 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    pub fn scale(&self, scale: f32) -> Vec3 {
        Vec3(self.0 * scale, self.1 * scale, self.2 * scale)
    }

    #[must_use]
    pub fn normalize(&self) -> Vec3 {
        let len = self.len();
        Vec3(self.0 / len, self.1 / len, self.2 / len)
    }

    #[must_use]
    pub fn rotate(&self, yaw: f32, pitch: f32, roll: f32) -> Vec3 {
        let mat: [[f32; 3]; 3] = [
            [
                yaw.cos() * roll.cos(),
                pitch.sin() * yaw.sin() * roll.cos() - pitch.cos() * roll.sin(),
                pitch.cos() * yaw.sin() * roll.cos() + pitch.sin() * roll.sin(),
            ],
            [
                yaw.cos() * roll.sin(),
                pitch.sin() * yaw.sin() * roll.sin() + pitch.cos() * roll.cos(),
                pitch.cos() * yaw.sin() * roll.sin() - pitch.sin() * roll.cos(),
            ],
            [-yaw.sin(), pitch.sin() * yaw.cos(), pitch.cos() * yaw.cos()],
        ];
        Vec3(
            self.0 * mat[0][0] + self.1 * mat[0][1] + self.2 * mat[0][2],
            self.0 * mat[1][0] + self.1 * mat[1][1] + self.2 * mat[1][2],
            self.0 * mat[2][0] + self.1 * mat[2][1] + self.2 * mat[2][2],
        )
    }
    pub fn tuple(&self) -> (f32, f32, f32) {
        (self.0, self.1, self.2)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, rhs: &Vec3) -> bool {
        (self.0 - rhs.0).abs() < FLOAT_MIN_DIFF
            && (self.1 - rhs.1).abs() < FLOAT_MIN_DIFF
            && (self.2 - rhs.2).abs() < FLOAT_MIN_DIFF
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Vec3;

    #[test]
    fn vec3_test_add_1() {
        let a = Vec3(0.0, 0.0, 0.0);
        let b = Vec3(1.0, 1.0, 1.0);
        assert_eq!(a + b, b);
    }

    #[test]
    fn vec3_test_partial_eq() {
        let a = Vec3(0.0, 0.0, 0.0);
        assert_eq!(a, a);
    }

    #[test]
    fn vec3_rotate_right() {
        let a = Vec3(0.0, 0.0, 1.0).rotate(0.5 * std::f32::consts::PI, 0.0, 0.0);
        assert_eq!(a, Vec3(1.0, 0.0, 0.0));
    }

    #[test]
    fn vec3_rotate_up() {
        let a = Vec3(0.0, 0.0, 1.0).rotate(0.0, -0.5 * std::f32::consts::PI, 0.0);
        assert_eq!(a, Vec3(0.0, 1.0, 0.0));
    }

    #[test]
    fn vec3_rotate_completely() {
        let a = Vec3(1.0, 2.0, 3.0).rotate(
            2.0 * std::f32::consts::PI,
            2.0 * std::f32::consts::PI,
            2.0 * std::f32::consts::PI,
        );
        assert_eq!(a, Vec3(1.0, 2.0, 3.0));
    }
}
