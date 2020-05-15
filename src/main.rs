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
use rand::{thread_rng, Rng, RngCore};
use std::f64::consts::PI;

const IMAGE_RESOLUTION: i16 = 512;
const SAMPLES_PER_PIXEL: i16 = 10;

fn main() {
    let mut image_file = File::create("target/result.ppm").unwrap();
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
        position: Point3::new(-0., 100.5, -1.),
        radius: 100.,
    };
    let shapes = vec![sphere1.clone(), sphere2.clone()];
    write!(image_file, "P3\n{}\n{}\n255\n", image_width, image_height);
    let mut rng = thread_rng();
    for y in 0..image_height {
        // print!("\rScanlines remaining: {}", y);
        for x in 0..image_width {
            let mut pixel_color = Color::new(0., 0., 0.);
            for sample_index in 0..SAMPLES_PER_PIXEL {
                let u = (x as f64 + rng.gen_range(0., 1.)) / (image_width - 1) as f64;
                let v = (y as f64 + rng.gen_range(0., 1.)) / (image_height - 1) as f64;
                let ray = Ray::new(camera.position,
                                   camera.film_lower_left_corner()
                                       + u * horizontal
                                       + v * vertical,
                );
                pixel_color += &ray_color(&ray, 50, &shapes, &mut rng);
            }
            pixel_color /= SAMPLES_PER_PIXEL as f64;
            image_file.write(pixel_color.ppm_color().as_bytes());
            // println!("x: {}, y: {}, color: {}", x, y, color.ppm_color())
        }
    }
}

fn ray_color(ray: &Ray, depth: i16, shapes: &Vec<Shape>, rng: &mut dyn RngCore) -> Color {
    if depth <= 0 {
        return Color::new(0., 0., 0.);
    }
    let hit_result_option = shapes.iter()
        .find_map(|shape| shape.hit(ray, 0.001, 200.));
    match hit_result_option {
        None => {
            let unit_direction = ray.direction.normalized();
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
        Some(hit_result) => {
            let new_diffuse_target = hit_result.position + hit_result.normal + random_unit_vector(rng);
            0.5 * ray_color(
                &Ray::new(hit_result.position, new_diffuse_target - hit_result.position),
                depth - 1,
                shapes,
                rng,
            )
        }
    }
}

fn random_unit_vector(rng: &mut dyn RngCore) -> Vec3 {
    let angle = rng.gen_range(0., 2. * PI);
    let z = rng.gen_range(-1., 1.);
    let r = f64::sqrt(1. - z * z);
    Vec3::new(r * angle.cos(), r * angle.sin(), z)
}
