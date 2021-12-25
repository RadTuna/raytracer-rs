
use crate::math::vec3::{Point3, Vec3};
use crate::ray::Ray;

pub struct Camera {
    origin: Point3,
    lower_left: Point3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new_default() -> Camera {
        Camera { 
            origin: Point3::new_default(), 
            lower_left: Point3::new(-1.0, -1.0, -1.0), 
            horizontal: Vec3::new(2.0, 0.0, 0.0), 
            vertical: Vec3::new(0.0, 2.0, 0.0) }
    }

    pub fn new(aspect_ratio: f64) -> Camera {
        let mut new_camera = Camera::new_default();
        new_camera.update(aspect_ratio);
        new_camera
    }

    pub fn update(&mut self, aspect_ratio: f64) {
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
    
        self.origin = Point3::new(0.0, 0.0, 0.0);
        self.horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        self.vertical = Vec3::new(0.0, viewport_height, 0.0);
        self.lower_left = self.origin - self.horizontal / 2.0 - self.vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left + self.horizontal * u + self.vertical * v - self.origin
        )
    }
}
