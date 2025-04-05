#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub struct Point3(pub f64, pub f64, pub f64);

impl Default for Point3 {
    fn default() -> Self {
        Point3(0f64, 0f64, 0f64)
    }
}
