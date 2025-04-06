use crate::{point3::Point3, ray::Ray, sphere::Sphere, vec3::Vec3};

#[derive(Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

#[allow(unused)]
impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add_sphere(&mut self, center: Point3, radius: f64) {
        let sphere = Sphere::new(center, radius);
        self.objects.push(Box::new(sphere));
    }

    fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_tmin: f64, mut ray_tmax: f64) -> Option<HitRecord> {
        let mut closet_rec = None;

        for obj in self.objects.iter() {
            if let Some(cur_rec) = obj.hit(r, ray_tmin, ray_tmax) {
                ray_tmax = cur_rec.t;
                closet_rec = Some(cur_rec);
            }
        }

        closet_rec
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            *outward_normal * -1.0
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}
