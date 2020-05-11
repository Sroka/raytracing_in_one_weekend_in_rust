use crate::point3::Point3;
use crate::vec3::Vec3;
use std::borrow::Borrow;

pub struct Camera {
    pub position: Point3,
    pub aspect_ratio: f64,
    pub film_distance: f64,
    pub film_width: f64,
    pub film_height: f64,
}

impl Camera {
    pub fn new(
        position: Point3,
        aspect_ratio: f64,
    ) -> Camera {
        let film_width = 4.0;
        Camera {
            position,
            aspect_ratio,
            film_distance: 1.0,
            film_width,
            film_height: film_width / aspect_ratio,
        }
    }

    pub fn film_lower_left_corner(&self) -> Point3 {
        self.position + Vec3::new(-self.film_width / 2.0, -self.film_height / 2.0, self.film_distance)
    }
}
