
use crate::material::{Material, ScatteredResult};
use crate::math::vec3::Color;
use crate::object::HitRecord;
use crate::ray::Ray;
use crate::math::vec3::Vec3;



pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new_default() -> Lambertian {
        Lambertian { albedo: Color::new(0.0, 0.0, 0.0) }
    }

    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record : &HitRecord) -> Option<ScatteredResult> {
        let mut scattered_direction = hit_record.normal + Vec3::rand_in_unit_sphere().get_normal();
        if scattered_direction.is_near_zero() {
            scattered_direction = hit_record.normal;
        }

        let scattered_ray = Ray::new(
            hit_record.point, 
            scattered_direction);

        let result = ScatteredResult { 
            attenuation: self.albedo, 
            scattered_ray 
        };

        Some(result)
    }
}
