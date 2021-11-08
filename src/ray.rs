
use super::math::vec3::Point3;
use super::math::vec3::Vec3;

pub struct Ray {
    origin: Point3,
    direction: Vec3
}

impl Ray {
    pub fn default() -> Ray {
        let origin = Point3::default();
        let direction = Vec3::default();
        Ray { origin, direction }
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


