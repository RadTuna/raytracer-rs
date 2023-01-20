#![allow(dead_code)]
#![allow(unused_variables)]

pub mod errormat;
pub mod lambertian;
pub mod metal;

use crate::math::vec3::Color;
use crate::object::HitRecord;
use crate::ray::Ray;


pub struct ScatteredResult {
    pub attenuation : Color,
    pub scattered_ray : Ray
}

pub trait Material: Send {
    fn scatter(&self, ray: &Ray, hit_record : &HitRecord) -> Option<ScatteredResult>;
}
