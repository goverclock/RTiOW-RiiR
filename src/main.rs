use std::fs;

mod color;
mod point3;
mod ray;
mod vec3;

use color::{Color, write_color};
use point3::Point3;
use ray::Ray;
use vec3::Vec3;

#[allow(unused)]
fn write_img(img_width: u64, img_height: u64) {
    let mut content = String::with_capacity(img_width as usize * img_height as usize * 12);
    content += &format!("P3\n{img_width} {img_height}\n255\n");

    for j in 0..img_height {
        for i in 0..img_width {
            let c = Color::from_scale(
                i as f64 / (img_width as f64 - 1f64),
                j as f64 / (img_height as f64 - 1f64),
                0.0,
            );
            write_color(&mut content, &c);
        }
        if j % 50 == 0 {
            println!("rendering line {j}");
        }
    }

    println!("render finished");
    fs::write("x.ppm", content).unwrap();
}

fn main() {
    // the image
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 400u64;
    let img_height = ((img_width as f64 / aspect_ratio) as u64).max(1);

    // camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (img_width as f64 / img_height as f64);
    let camera_center = Point3(0.0, 0.0, 0.0);

    let viewport_u = Vec3(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / img_width as f64;
    let pixel_delta_v = viewport_v / img_height as f64;

    let viewport_upper_left =
        camera_center - Vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // fs::remove_file("x.ppm").unwrap();
    // write_img(256, 256);
    let mut content = String::with_capacity(img_width as usize * img_height as usize * 12);
    content += &format!("P3\n{img_width} {img_height}\n255\n");

    for j in 0..img_height {
        for i in 0..img_width {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray {
                orig: camera_center,
                dir: ray_direction,
            };

            // let c = Color::from_scale(
            //     i as f64 / (img_width as f64 - 1f64),
            //     j as f64 / (img_height as f64 - 1f64),
            //     0.0,
            // );
            let c = ray.ray_color();
            write_color(&mut content, &c);
        }
        if j % 50 == 0 {
            println!("rendering line {j}");
        }
    }

    println!("render finished");
    fs::write("x.ppm", content).unwrap();
}
