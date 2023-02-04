
use crate::math::vec3::{Point3, Vec3};
use crate::ray::Ray;


#[derive(Clone, Copy)]
pub struct CameraSettings {
    fov: f64,
    aperture: f64,
    focus_dist: f64
}

impl CameraSettings {
    pub fn new_default() -> CameraSettings {
        CameraSettings { 
            fov: 90.0, 
            aperture: 0.1, 
            focus_dist: 10.0
        }
    }

    pub fn new(fov: f64, aperture: f64, focus_dist: f64) -> CameraSettings {
        CameraSettings { fov, aperture, focus_dist }
    }
}

#[derive(Clone)]
pub struct Camera {
    origin: Point3,
    lower_left: Point3,
    horizontal: Vec3,
    vertical: Vec3,

    view_forward: Vec3,
    view_right: Vec3,
    view_up: Vec3,
    lens_radius: f64
}

impl Camera {
    pub fn new_default() -> Camera {
        Camera { 
            origin: Point3::new_default(), 
            lower_left: Point3::new(-1.0, -1.0, -1.0), 
            horizontal: Vec3::new(2.0, 0.0, 0.0), 
            vertical: Vec3::new(0.0, 2.0, 0.0),
            view_forward: Vec3::new(0.0, 0.0, 1.0),
            view_right: Vec3::new(1.0, 0.0, 0.0),
            view_up: Vec3::new(0.0, 1.0, 0.0),
            lens_radius: 0.1
        }
    }

    pub fn new(look_from: Point3, look_to: Point3, aspect_ratio: f64, settings: CameraSettings) -> Camera {
        let mut new_camera = Camera::new_default();
        new_camera.update(look_from, look_to, aspect_ratio, settings);
        new_camera
    }

    pub fn update(&mut self, look_from: Point3, look_to: Point3, aspect_ratio: f64, settings: CameraSettings) {
        let theta = settings.fov.to_radians();
        let h = (theta * 0.5).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        self.view_up = Vec3::new(0.0, 1.0, 0.0);
        self.view_forward = (look_from - look_to).get_normal();
        self.view_right = Vec3::cross(&self.view_up, &self.view_forward).get_normal();
        self.view_up = Vec3::cross(&self.view_forward, &self.view_right);
    
        self.origin = look_from;
        self.horizontal = self.view_right * viewport_width * settings.focus_dist;
        self.vertical = self.view_up * viewport_height * settings.focus_dist;
        self.lower_left = self.origin - self.horizontal / 2.0 - self.vertical / 2.0 - self.view_forward * settings.focus_dist;
        self.lens_radius = settings.aperture / 2.0;
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rand_disk = Vec3::rand_in_unit_disk() * self.lens_radius;
        let offset = self.view_right * rand_disk.x + self.view_up * rand_disk.y;

        Ray::new(
            self.origin + offset,
            self.lower_left + self.horizontal * u + self.vertical * v - self.origin - offset
        )
    }
}
