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

    pub fn ray_color(&self, world: &HittableList) -> Color {
        if let Some(rec) = world.hit(self, 0.0, f64::MAX) {
            // let light_source = Point3(1.0, 1.0, -1.0);
            // let light_path = rec.p - light_source;
            // let brightness = light_path.length();
            // let brightness_scale = if brightness > 1.0 {
            //     1.0 / brightness
            // } else {
            //     1.0
            // };

            // return Color::from_scale(
            //     0.5 * (brightness_scale + 1.0),
            //     0.5 * (brightness_scale + 1.0),
            //     0.5 * (brightness_scale + 1.0),
            // );

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
}
