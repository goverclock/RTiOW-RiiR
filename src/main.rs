mod color;
mod hittable;
mod point3;
mod ray;
mod render;
mod sphere;
mod vec3;

use hittable::HittableList;
use point3::Point3;
use render::Render;

fn main() {
    let mut world = HittableList::default();

    world.add_sphere(Point3(0.0, 0.0, -1.0), 0.5);
    world.add_sphere(Point3(0.0, -100.5, -1.0), 100.0);
    world.add_sphere(Point3(2.1, 1.4, -3.0), 1.0);

    let render = Render::new(400u64, 16.0 / 9.0, 1.0, 2.0);
    render.run(&world);
}
