use crate::color::Color;
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

    pub fn ray_color(&self) -> Color {
        let unit_dir = self.dir.unit_vector();
        let a = unit_dir.1 * 0.5 + 1.0;
        let c1 = Color::from_scale(1.0, 1.0, 1.0);
        let c2 = Color::from_scale(0.5, 0.7, 1.0);

        Color::from_mix((c1, 1.0 - a), (c2, a))
    }
}
