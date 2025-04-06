use std::f64;

use crate::color::Color;
use crate::hittable::{Hittable, HittableList};
use crate::point3::Point3;
use crate::vec3::Vec3;

#[allow(unused)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

#[allow(unused)]
impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }

    pub fn ray_color(&self, world: &HittableList) -> Color {
        if let Some(rec) = world.hit(self, 0.0, f64::MAX) {
            return Color::from_scale(
                0.5 * (rec.normal.0 + 1.0),
                0.5 * (rec.normal.1 + 1.0),
                0.5 * (rec.normal.2 + 1.0),
            );
        }

        let unit_dir = self.dir.unit_vector();
        let a = unit_dir.1 * 0.5 + 1.0;
        let c1 = Color::from_scale(1.0, 1.0, 1.0);
        let c2 = Color::from_scale(0.5, 0.7, 1.0);

        Color::from_mix((c1, 1.0 - a), (c2, a))
    }

    fn hit_sphere(&self, center: Point3, r: f64) -> Option<f64> {
        let oc = center - self.orig;
        let a = self.dir.length_squared();
        let h = self.dir.dot(&oc);
        let c = oc.dot(&oc) - r * r;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            None
        } else {
            Some((h - f64::sqrt(discriminant)) / a)
        }
    }
}
