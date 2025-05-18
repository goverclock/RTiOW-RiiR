use crate::color::{Color, write_color};
use crate::hittable::HittableList;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::Rng;
use std::fs;

pub struct Render {
    img_width: u64,
    img_height: u64,
    samples_per_pixel: usize,
    max_depth: usize,
    _focal_length: f64,
    _viewport_height: f64,
    _viewport_width: f64,
    camera_center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Render {
    pub fn new(img_width: u64, aspect_ratio: f64, focal_length: f64, viewport_height: f64) -> Self {
        // the image
        let img_height = ((img_width as f64 / aspect_ratio) as u64).max(1);

        // camera
        let viewport_width = viewport_height * (img_width as f64 / img_height as f64);
        let camera_center = Point3(0.0, 0.0, 0.0);

        let viewport_u = Vec3(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / img_width as f64;
        let pixel_delta_v = viewport_v / img_height as f64;

        let viewport_upper_left =
            camera_center - Vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Self {
            img_width,
            img_height,
            samples_per_pixel: 10,
            max_depth: 50,
            _focal_length: focal_length,
            _viewport_height: viewport_height,
            _viewport_width: viewport_width,
            camera_center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    fn get_random_sample_ray(&self, i: u64, j: u64) -> Ray {
        let i = i as f64 + rand::rng().random_range(-0.5..=0.5);
        let j = j as f64 + rand::rng().random_range(-0.5..=0.5);
        let pixel_sample = self.pixel00_loc + self.pixel_delta_u * i + self.pixel_delta_v * j;

        Ray {
            orig: self.camera_center,
            dir: pixel_sample - self.camera_center,
        }
    }

    pub fn run(&self, world: &HittableList) {
        let mut content =
            String::with_capacity(self.img_width as usize * self.img_height as usize * 12);
        content += &format!("P3\n{} {}\n255\n", self.img_width, self.img_height);

        for j in 0..self.img_height {
            for i in 0..self.img_width {
                let mut colors = vec![];
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_random_sample_ray(i, j);
                    colors.push(ray.ray_color(self.max_depth, world));
                }
                write_color(&mut content, &Color::average(colors));
            }
            if j % 50 == 0 {
                println!("rendering line {j}");
            }
        }

        println!("render finished");
        fs::write("x.ppm", content).unwrap();
    }
}
