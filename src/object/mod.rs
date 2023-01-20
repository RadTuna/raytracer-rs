#![allow(dead_code)]
#![allow(unused_variables)]

pub mod sphere;

use crate::math::vec3::{Vec3, Point3};
use crate::ray::Ray;
use crate::material::Material;

pub struct HitRecord<'a> {
    pub point: Point3,
    pub normal: Vec3,
    pub weight: f64,
    pub is_front_face: bool,
    pub material : &'a dyn Material
}

impl<'a> HitRecord<'a> {
    pub fn set_face_from_ray(&mut self, ray: &Ray) {
        self.is_front_face = Vec3::dot(ray.get_direction(), &self.normal) < 0.0;
        if self.is_front_face == false {
            self.normal = -self.normal;
        }
    }
}

pub trait Hittable: Send {
    fn hit(&self, ray: &Ray, weight_min: f64, weight_max: f64) -> Result<HitRecord, ()>;
}
