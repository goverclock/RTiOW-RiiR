use std::fmt::Write;
use std::fs;

fn main() {
    let img_width = 256;
    let img_height = 256;

    let mut content = format!("P3\n{img_width} {img_height}\n255\n");
    for j in 0..img_height {
        for i in 0..img_width {
            let r = i as f64 / (img_width as f64 - 1f64);
            let g = j as f64 / (img_height as f64 - 1f64);
            let b = 0.0;

            let ir = (r * 255.999) as u64;
            let ig = (g * 255.999) as u64;
            let ib = (b * 255.999) as u64;
            write!(&mut content, "{ir} {ig} {ib}\n").unwrap();
        }
    }

    fs::write("x.ppm", content).unwrap();
}
