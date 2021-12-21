
pub mod sphere;

use super::math::vec3::{Vec3, Point3};
use super::ray::Ray;

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub weight: f64,
    pub is_front_face: bool
}

impl HitRecord {
    pub fn set_face_from_ray(&mut self, ray: &Ray) {
        self.is_front_face = Vec3::dot(ray.get_direction(), &self.normal) < 0.0;
        if self.is_front_face == false {
            self.normal = -self.normal;
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, weight_min: f64, weight_max: f64) -> Result<HitRecord, ()>;
}
