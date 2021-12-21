
use crate::math::vec3::{Point3, Vec3};

pub struct Ray {
    origin: Point3,
    direction: Vec3
}

impl Ray {
    pub fn new_default() -> Ray {
        Ray::new(Point3::new_default(), Vec3::new_default())
    }

    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn get_origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn get_direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn get_point(&self, weight: f64) -> Point3 {
        self.origin + self.direction * weight
    }
}


