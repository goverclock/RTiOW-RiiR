use std::fs;

mod color;
mod point3;
mod vec3;

use color::{Color, write_color};
use vec3::Vec3;

#[allow(unused)]
fn write_img(img_width: u64, img_height: u64, _pixels: String) {
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
    fs::remove_file("x.ppm").unwrap();
    write_img(256, 256, String::new());

    let mut v1 = Vec3(3.0, 4.0, 0.0);
    let v2 = Vec3(1.0, 2.0, 3.0);
    v1 *= 3.0;

    println!("{v1:#?}");
    println!("{}", v1.length());
    println!("{}", v1 + v2);
}
