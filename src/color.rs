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

    pub fn from_mix(c1: (Color, f64), c2: (Color, f64)) -> Color {
        let r = (c1.0.0 as f64 * c1.1 + c2.0.0 as f64 * c2.1) as u8;
        let g = (c1.0.1 as f64 * c1.1 + c2.0.1 as f64 * c2.1) as u8;
        let b = (c1.0.2 as f64 * c1.1 + c2.0.2 as f64 * c2.1) as u8;
        Color(r, g, b)
    }
}

pub fn write_color(buf: &mut String, c: &Color) {
    writeln!(buf, "{} {} {}", c.0, c.1, c.2).unwrap();
}
