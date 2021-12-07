
pub mod math;
mod ray;

use math::vec3::Vec3;
use math::vec3::Color;

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0 .. image_height).rev() {
        eprintln!("Scanline remaining: {}", j);

        for i in 0 .. image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v - origin);
            let pixel_color = ray_color(&ray);

            Color::write_color(&pixel_color);
        }
    }
}


use ray::Ray;

use crate::math::vec3::Point3;

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
