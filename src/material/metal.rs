
use crate::material::{Material, ScatteredResult};
use crate::math::vec3::{Vec3, Color};
use crate::object::HitRecord;
use crate::ray::Ray;


#[derive(Clone)]
pub struct Metal {
    albedo: Color,
    fuzziness: f64
}

impl Metal {
    pub fn new_default() -> Metal {
        Metal { 
            albedo: Color::new(0.0, 0.0, 0.0),
            fuzziness: 0.0
        }
    }

    pub fn new(albedo: Color, fuzziness: f64) -> Metal {
        let clamped_fuzziness = fuzziness.clamp(0.0, 1.0);
        Metal { 
            albedo, 
            fuzziness: clamped_fuzziness
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record : &HitRecord) -> Option<ScatteredResult> {
        let unit_ray_direction = ray.get_direction().get_normal();
        let mut reflected_direction = unit_ray_direction.reflect(&hit_record.normal);

        let fuzzy_vector = Vec3::rand_in_unit_sphere() * self.fuzziness;
        reflected_direction += fuzzy_vector;
        reflected_direction.normalize();

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

