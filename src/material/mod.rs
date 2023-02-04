
pub mod errormat;
pub mod lambertian;
pub mod metal;

use dyn_clone::DynClone;

use crate::math::vec3::Color;
use crate::object::HitRecord;
use crate::ray::Ray;


pub struct ScatteredResult {
    pub attenuation : Color,
    pub scattered_ray : Ray
}

pub trait Material: Send + DynClone {
    fn scatter(&self, ray: &Ray, hit_record : &HitRecord) -> Option<ScatteredResult>;
}

dyn_clone::clone_trait_object!(Material);
