use crate::point3::Point3;
use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

pub struct HitResult {
    pub position: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction,
        }
    }

    pub fn at(&self, time: f64) -> Point3 {
        self.origin + self.direction * time
    }
}
