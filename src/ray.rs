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

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }

    pub fn ray_color(&self, depth: usize, world: &HittableList) -> Color {
        if depth <= 0 {
            return Color(0, 0, 0);
        }

        if let Some(rec) = world.hit(self, 0.01, f64::MAX) {
            let direction = rec.normal + Vec3::random_unit_vector();
            let r = Ray {
                orig: rec.p,
                dir: direction,
            };
            let c = r.ray_color(depth - 1, world);

            return Color(c.0 / 2, c.1 / 2, c.2 / 2);
        }

        let unit_dir = self.dir.unit_vector();
        let a = unit_dir.1 * 0.5 + 1.0;
        let c1 = Color::from_scale(1.0, 1.0, 1.0);
        let c2 = Color::from_scale(0.5, 0.7, 1.0);

        Color::from_mix((c1, 1.0 - a), (c2, a))
    }
}
