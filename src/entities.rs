use crate::shapes::Shape;
use crate::material::Material;
use crate::vec3::Vec3;
use crate::point3::Point3;
use crate::ray::{Ray, HitResult};

#[derive(Copy, Clone)]
pub struct Entity {
    pub shape: Shape,
    pub material: Material,
}

impl Entity {
    pub fn new(
        shape: Shape,
        material: Material,
    ) -> Entity {
        Entity {
            shape,
            material,
        }
    }
    pub fn position(&self) -> Point3 {
        match self.shape { Shape::Sphere { position, radius } => { position } }
    }
}

impl Entity {
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
        return match self.shape {
            Shape::Sphere { position, radius } => {
                let distance = ray.origin - position;
                let a = Vec3::dot(&ray.direction, &ray.direction);
                let b = 2.0 * Vec3::dot(&distance, &ray.direction);
                let c = Vec3::dot(&distance, &distance) - radius * radius;
                let discriminant = b * b - 4.0 * a * c;
                if discriminant > 0. {
                    let discriminant_sqrt = discriminant.sqrt();
                    let smaller_root = (-b - discriminant_sqrt) / (2. * a);
                    if smaller_root > t_min && smaller_root < t_max {
                        let hit_position = ray.at(smaller_root);
                        let outward_normal = ((hit_position - position) / radius).normalized();
                        let front_face = Vec3::dot(&ray.direction, &outward_normal) < 0.;
                        return Option::Some(HitResult {
                            position: hit_position,
                            normal: if front_face { outward_normal } else { -outward_normal },
                            t: smaller_root,
                            front_face,
                        });
                    }
                    let larger_root = (-b + discriminant_sqrt) / (2. * a);
                    if larger_root > t_min && larger_root < t_max {
                        let hit_position = ray.at(larger_root);
                        let outward_normal = ((hit_position - position) / radius).normalized();
                        let front_face = Vec3::dot(&ray.direction, &outward_normal) < 0.;
                        return Option::Some(HitResult {
                            position: hit_position,
                            normal: if front_face { outward_normal } else { -outward_normal },
                            t: larger_root,
                            front_face,
                        });
                    }
                }
                Option::None
            }
        };
    }
}
