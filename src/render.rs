use crate::color::write_color;
use crate::hittable::HittableList;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::fs;

pub struct Render {
    img_width: u64,
    img_height: u64,
    focal_length: f64,
    viewport_height: f64,
}

impl Render {
    pub fn new(img_width: u64, aspect_ratio: f64, focal_length: f64, viewport_height: f64) -> Self {
        // the image
        let img_height = ((img_width as f64 / aspect_ratio) as u64).max(1);

        // camera
        Self {
            img_width,
            img_height,
            focal_length,
            viewport_height,
        }
    }

    pub fn run(&self, world: &HittableList) {
        fs::remove_file("x.ppm").unwrap();
        let mut content =
            String::with_capacity(self.img_width as usize * self.img_height as usize * 12);
        content += &format!("P3\n{} {}\n255\n", self.img_width, self.img_height);

        let viewport_width =
            self.viewport_height * (self.img_width as f64 / self.img_height as f64);
        let camera_center = Point3(0.0, 0.0, 0.0);

        let viewport_u = Vec3(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3(0.0, -self.viewport_height, 0.0);

        let pixel_delta_u = viewport_u / self.img_width as f64;
        let pixel_delta_v = viewport_v / self.img_height as f64;

        let viewport_upper_left =
            camera_center - Vec3(0.0, 0.0, self.focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        for j in 0..self.img_height {
            for i in 0..self.img_width {
                let pixel_center =
                    pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
                let ray_direction = pixel_center - camera_center;
                let ray = Ray {
                    orig: camera_center,
                    dir: ray_direction,
                };

                let c = ray.ray_color(world);
                write_color(&mut content, &c);
            }
            if j % 50 == 0 {
                println!("rendering line {j}");
            }
        }

        println!("render finished");
        fs::write("x.ppm", content).unwrap();
    }
}
