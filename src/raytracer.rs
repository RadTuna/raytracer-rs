
use std::io::Cursor;

use super::math::vec3::Vec3;
use super::math::vec3::Color;
use super::math::vec3::Point3;
use super::ray::Ray;

fn ray_color(ray: &Ray) -> Color {
    let weight = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, ray);
    if weight > 0.0 {
        let normal = (ray.get_point(weight) - Vec3::new(0.0, 0.0, -1.0)).get_normal();
        return Color::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0) * 0.5;
    }

    let unit_direction = ray.get_direction().get_normal();
    let weight = 0.5 * (unit_direction.y + 1.0);
    let result = 
        Color::new(1.0, 1.0, 1.0) * (1.0 - weight) 
        + Color::new(0.5, 0.7, 1.0) * weight;

    result
}

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let to_ray_origin = *ray.get_origin() - *center;
    let value_a = Vec3::dot(ray.get_direction(), ray.get_direction());
    let value_b = 2.0 * Vec3::dot(&to_ray_origin, ray.get_direction());
    let value_c = Vec3::dot(&to_ray_origin, &to_ray_origin) - radius * radius;
    let discriminant = value_b * value_b - 4.0 * value_a * value_c;
    if discriminant < 0.0 {
        -1.0
    } else {
        // quadratic formula
        (-value_b - discriminant.sqrt()) / (2.0 * value_a)
    }
}

pub struct RayTracer {
    buffer: Vec<u8>,
    size: (usize, usize),
    byte_size: usize
}

impl RayTracer {
    pub fn new(init_size: (usize, usize)) -> RayTracer {
        let mut new_buffer: Vec<u8> = Vec::new();
        let new_byte_size = init_size.0 * init_size.1 * 3;
        new_buffer.resize(new_byte_size, 0);

        RayTracer {
            buffer: new_buffer,
            size: (init_size.0, init_size.1),
            byte_size: new_byte_size
        }
    }

    pub fn run(&mut self) {
        // Camera
        let aspect_ratio = self.size.0 as f64 / self.size.1 as f64;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
    
        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    
        for y in (0 .. self.size.1).rev() {
            for x in 0 .. self.size.0 {
                let u = x as f64 / (self.size.0 - 1) as f64;
                let v = y as f64 / (self.size.1 - 1) as f64;
    
                let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v - origin);
                let pixel_color = ray_color(&ray);

                self.set_buffer((x, y), &pixel_color);
            }
        }

        print!("End raytrace!");
    }

    pub fn set_buffer(&mut self, pos: (usize, usize), color: &Color) {
        let index: usize = (self.size.0 * pos.1 * 3 + pos.0 * 3) as usize;
        self.buffer[index + 0] = (color[0] * 255.0) as u8;
        self.buffer[index + 1] = (color[1] * 255.0) as u8;
        self.buffer[index + 2] = (color[2] * 255.0) as u8;
    }

    pub fn get_buffer(&self) -> &Vec<u8> {
        &self.buffer
    }

    pub fn get_size(&self) -> &(usize, usize) {
        &self.size
    }
}

