
use crate::material::Material;
use crate::material::errormat::ErrorMat;
use crate::object::{Hittable, HitRecord};
use crate::math::vec3::{Vec3, Point3};
use crate::ray::Ray;


#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Box<dyn Material>
}

impl Sphere {
    pub fn new_default() -> Sphere {
        Sphere::new(
            Point3::new_default(), 
            0.0, 
            Box::new(ErrorMat::new_default()))
    }

    pub fn new(center: Point3, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, weight_min: f64, weight_max: f64) -> Result<HitRecord, ()>
    {
        let to_ray_origin = *ray.get_origin() - self.center;
        let value_a = ray.get_direction().sqaure_length();
        let value_half_b = Vec3::dot(&to_ray_origin, ray.get_direction());
        let value_c = to_ray_origin.sqaure_length() - self.radius * self.radius;
        let discriminant = value_half_b * value_half_b - value_a * value_c;
        if discriminant < 0.0 {
            return Err(());
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-value_half_b - sqrtd) / value_a; // near
        if root < weight_min || root > weight_max {
            root = (-value_half_b + sqrtd) / value_a; // far
            if root < weight_min || root > weight_max {
                return Err(());
            }
        }

        let hit_point = ray.get_point(root);
        let mut record = HitRecord {
            point: hit_point,
            normal: (hit_point - self.center) / self.radius,
            weight: root,
            is_front_face: true,
            material: &*self.material
        };
        record.set_face_from_ray(ray);

        Ok(record)
    }
}