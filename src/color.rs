use std::fmt::Write;

#[allow(unused)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub fn from_scale(r: f64, g: f64, b: f64) -> Self {
        Self(
            (r * 255.999) as u8,
            (g * 255.999) as u8,
            (b * 255.999) as u8,
        )
    }
}

pub fn write_color(buf: &mut String, c: &Color) {
    writeln!(buf, "{} {} {}", c.0, c.1, c.2).unwrap();
}
