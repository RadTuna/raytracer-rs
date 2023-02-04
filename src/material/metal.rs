
use crate::material::{Material, ScatteredResult};
use crate::math::vec3::Color;
use crate::object::HitRecord;
use crate::ray::Ray;


#[derive(Clone)]
pub struct Metal {
    albedo: Color
}

impl Metal {
    pub fn new_default() -> Metal {
        Metal { albedo: Color::new(0.0, 0.0, 0.0) }
    }

    pub fn new(albedo: Color) -> Metal {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record : &HitRecord) -> Option<ScatteredResult> {
        let unit_ray_direction = ray.get_direction().get_normal();
        let reflected_direction = unit_ray_direction.relfect(&hit_record.normal);

        let scattered_ray = Ray::new(
            hit_record.point, 
            reflected_direction);

        let result = ScatteredResult { 
            attenuation: self.albedo, 
            scattered_ray
        };

        Some(result)
    }
}

