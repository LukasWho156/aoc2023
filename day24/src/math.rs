// probably overkill, but fun. Also, I know this could be done
// more elegantly with macros, but that does seem like total overkill
// right now.

use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

pub struct Mat2x3 {
    pub a00: f64,
    pub a01: f64,
    pub a02: f64,
    pub a10: f64,
    pub a11: f64,
    pub a12: f64,
}

pub trait Vector {
    fn is_parallel(&self, other: &Self) -> bool;
    fn absolute(&self) -> f64;
    fn normalize(&mut self);
    fn get_normalized(&self) -> Self;
}

#[derive(PartialEq, Debug)]
pub struct Vec3f {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector for Vec3f {
    fn is_parallel(&self, other: &Self) -> bool {
        (self.x * other.y * other.z == self.y * other.x * other.z)
            && (self.y * other.x * other.z == self.x * other.y * other.z)
    }

    fn absolute(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn normalize(&mut self) {
        let norm = 1.0 / self.absolute();
        self.x *= norm;
        self.y *= norm;
        self.z *= norm;
    }

    fn get_normalized(&self) -> Self {
        let norm = 1.0 / self.absolute();
        Vec3f {
            x: self.x * norm,
            y: self.y * norm,
            z: self.z * norm,
        }
    }

}

impl Vec3f {
    pub fn project(&self, mat: &Mat2x3) -> Vec2f {
        Vec2f {
            x: mat.a00 * self.x + mat.a01 * self.y + mat.a02,
            y: mat.a10 * self.x + mat.a11 * self.y + mat.a12,
        }
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vec3f {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Add<&Self> for Vec3f {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self::Output {
        Vec3f {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<Self> for &Vec3f {
    type Output = Vec3f;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3f {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign<&Self> for Vec3f {
    fn add_assign(&mut self, rhs: &Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<&Self> for Vec3f {
    type Output = Self;
    fn sub(self, rhs: &Self) -> Self::Output {
        Vec3f {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<Self> for &Vec3f {
    type Output = Vec3f;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3f {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign<&Self> for Vec3f {
    fn sub_assign(&mut self, rhs: &Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<&Self> for Vec3f {
    type Output = f64;
    fn mul(self, rhs: &Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Mul<Self> for &Vec3f {
    type Output = f64;
    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Mul<f64> for Vec3f {
    type Output = Vec3f;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3f {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<f64> for &Vec3f {
    type Output = Vec3f;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3f {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f64> for Vec3f {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Vec3f {
    type Output = Vec3f;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3f {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div<f64> for &Vec3f {
    type Output = Vec3f;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3f {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Vec3f {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

#[derive(PartialEq, Debug)]
pub struct Vec2f {
    pub x: f64,
    pub y: f64,
}

impl Vector for Vec2f {
    fn is_parallel(&self, other: &Self) -> bool {
        self.x * other.y == self.y * other.x
    }

    fn absolute(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn normalize(&mut self) {
        let norm = 1.0 / self.absolute();
        self.x *= norm;
        self.y *= norm;
    }

    fn get_normalized(&self) -> Self {
        let norm = 1.0 / self.absolute();
        Vec2f {
            x: self.x * norm,
            y: self.y * norm,
        }
    }

}

impl Add<&Self> for Vec2f {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self::Output {
        Vec2f {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Self> for &Vec2f {
    type Output = Vec2f;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2f {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<&Self> for Vec2f {
    fn add_assign(&mut self, rhs: &Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<&Self> for Vec2f {
    type Output = Self;
    fn sub(self, rhs: &Self) -> Self::Output {
        Vec2f {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<Self> for &Vec2f {
    type Output = Vec2f;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2f {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<&Self> for Vec2f {
    fn sub_assign(&mut self, rhs: &Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<&Self> for Vec2f {
    type Output = f64;
    fn mul(self, rhs: &Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl Mul<Self> for &Vec2f {
    type Output = f64;
    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl Mul<f64> for Vec2f {
    type Output = Vec2f;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec2f {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<f64> for &Vec2f {
    type Output = Vec2f;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec2f {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<f64> for Vec2f {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<f64> for Vec2f {
    type Output = Vec2f;
    fn div(self, rhs: f64) -> Self::Output {
        Vec2f {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Div<f64> for &Vec2f {
    type Output = Vec2f;
    fn div(self, rhs: f64) -> Self::Output {
        Vec2f {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<f64> for Vec2f {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

#[derive(Copy, Clone, Debug)]
struct FixedFloat {
    integer: i64,
    decimal: f64,
}

impl From<f64> for FixedFloat {
    fn from(value: f64) -> Self {
        let (integer, decimal) = match value < 0.0 {
            true => (value.trunc() as i64 - 1, value.fract() + 1.0),
            false => (value.trunc() as i64, value.fract()),
        };
        FixedFloat { integer, decimal }
    }
}

impl Add<Self> for FixedFloat {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let r_d = self.decimal + rhs.decimal;
        let integer = self.integer + rhs.integer + r_d.trunc() as i64;
        let decimal = r_d.fract();
        FixedFloat { integer, decimal }
    }
}

impl Sub<Self> for FixedFloat {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let r_d = self.decimal - rhs.decimal;
        let (integer, decimal) = match r_d < 0.0 {
            true => (self.integer - rhs.integer + r_d.trunc() as i64 - 1, r_d.fract() + 1.0),
            false => (self.integer - rhs.integer + r_d.trunc() as i64, r_d.fract())
        };
        FixedFloat { integer, decimal }
    }
}