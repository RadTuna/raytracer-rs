
use rand::{thread_rng, Rng};

use crate::material::{Material, ScatteredResult};
use crate::math::vec3::{Color, Vec3};
use crate::object::HitRecord;
use crate::ray::Ray;


#[derive(Clone)]
pub struct Dielectric {
    refraction_index: f64
}

impl Dielectric {
    pub fn new_default() -> Dielectric {
        Dielectric::new(0.0)
    }

    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric { refraction_index }
    }

    fn reflectance(cos: f64, refract_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let mut r0 = (1.0 - refract_idx) / (1.0 + refract_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cos).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record : &HitRecord) -> Option<ScatteredResult> {
        let refraction_ratio = if hit_record.is_front_face { 1.0 / self.refraction_index } else { self.refraction_index };
        let ray_direction = ray.get_direction().get_normal();
        let inv_ray_direction = ray_direction * -1.0;
        let cos_theta = Vec3::dot(&inv_ray_direction, &hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let mut rng = thread_rng();
        let cannot_refract = 
            refraction_ratio * sin_theta > 1.0 ||
            Dielectric::reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0 .. 1.0);

        let refracted = if cannot_refract {
            ray_direction.reflect(&hit_record.normal)
        } else {
            ray_direction.refract(&hit_record.normal, refraction_ratio)
        };

        let albedo = Color::new(1.0, 1.0, 1.0);
        let scattered_ray = Ray::new(
            hit_record.point, 
            refracted);

        let result = ScatteredResult { 
            attenuation: albedo, 
            scattered_ray
        };

        Some(result)
    }
}
