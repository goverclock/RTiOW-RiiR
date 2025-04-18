use crate::vec3::Vec3;
use std::ops::{Add, AddAssign, Sub};

#[allow(unused)]
#[derive(Default, Debug, Clone, Copy)]
pub struct Point3(pub f64, pub f64, pub f64);

impl Add<Vec3> for Point3 {
    type Output = Point3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub<Vec3> for Point3 {
    type Output = Point3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        self.add(rhs * -1.0)
    }
}

impl Sub<Point3> for Point3 {
    type Output = Vec3;
    fn sub(self, rhs: Point3) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl AddAssign<Vec3> for Point3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}
