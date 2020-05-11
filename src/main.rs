use std::fs::File;
use std::io::Write;

mod vec3;
mod color;
mod point3;
mod ray;
mod camera;
mod shapes;

use vec3::Vec3;
use shapes::Shape;
use crate::camera::Camera;
use crate::point3::Point3;
use crate::ray::{Ray, HitResult};
use crate::color::Color;

const IMAGE_RESOLUTION: i16 = 256;

fn main() {
    let mut image_file = File::create("result.ppm").unwrap();
    let camera = Camera::new(Point3::new(0.0, 0.0, 1.0), 16.0 / 9.0);
    let image_width = IMAGE_RESOLUTION;
    let image_height = (IMAGE_RESOLUTION as f64 / camera.aspect_ratio) as i16;
    let horizontal = Vec3::new(camera.film_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, camera.film_height, 0.0);
    let sphere1 = Shape::Sphere {
        position: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };
    let sphere2 = Shape::Sphere {
        position: Point3::new(-1., -1., -1.),
        radius: 0.25,
    };
    let shapes = vec![sphere1.clone(), sphere2.clone()];
    write!(image_file, "P3\n{}\n{}\n255\n", image_width, image_height);
    for y in (0..image_height).rev() {
        // print!("\rScanlines remaining: {}", y);
        for x in 0..image_width {
            let u = x as f64 / (image_width - 1) as f64;
            let v = y as f64 / (image_height - 1) as f64;
            let ray = Ray::new(camera.position,
                               camera.film_lower_left_corner()
                                   + u * horizontal
                                   + v * vertical,
            );

            let pixel_color = ray_color(&ray, &shapes);
            image_file.write(pixel_color.ppm_color().as_bytes());
            // println!("x: {}, y: {}, color: {}", x, y, color.ppm_color())
        }
    }
}

fn ray_color(ray: &Ray, shapes: &Vec<Shape>) -> Color {
    let hit_result_option = shapes.iter()
        .find_map(|shape| shape.hit(ray, 0., 200.));
    match hit_result_option {
        None => {
            let unit_direction = ray.direction.normalized();
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
        Some(hitResult) => {
            0.5 * Color::new(hitResult.normal.x + 1.0, hitResult.normal.y + 1.0, hitResult.normal.z + 1.0)
        }
    }
}
