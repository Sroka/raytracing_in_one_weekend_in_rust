use crate::point3::Point3;
use crate::ray::{Ray, HitResult};
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub enum Shape {
    Sphere { position: Point3, radius: f64 },
}
