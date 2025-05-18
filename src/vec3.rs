use std::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub},
};

use rand::Rng;

#[derive(Default, Debug, Clone, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl AddAssign<Self> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.mul_assign(1f64 / rhs);
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        self.add(rhs * -1.0)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        self.mul(1.0 / rhs)
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vec3({}, {}, {})", self.0, self.1, self.2)
    }
}

#[allow(unused)]
impl Vec3 {
    pub fn random_unit_vector() -> Self {
        loop {
            let p = Self::random(-1f64, 1f64);
            let lensq = p.length_squared();
            if lensq <= 1f64 && 1e-160 < lensq {
                break p / f64::sqrt(lensq);
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Self::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            on_unit_sphere * -1f64
        }
    }

    pub fn random_01() -> Self {
        Self(
            rand::rng().random_range(0f64..1f64),
            rand::rng().random_range(0f64..1f64),
            rand::rng().random_range(0f64..1f64),
        )
    }

    pub fn random(min: f64, max: f64) -> Self {
        Self(
            rand::rng().random_range(min..=max),
            rand::rng().random_range(min..=max),
            rand::rng().random_range(min..=max),
        )
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(&self, rhs: &Self) -> Vec3 {
        Self(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }
}
