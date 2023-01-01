
use crate::UserEvent;
use crate::math::vec3::{Vec3, Color, Point3};
use crate::ray::Ray;
use crate::world::World;
use crate::object::sphere::Sphere;
use crate::camera::Camera;
use rand::Rng;
use speedy2d::window::UserEventSender;



pub struct RayTracer {
    event_sender: UserEventSender<UserEvent>,
    buffer: Vec<u8>,
    buffer_size: (usize, usize),
    buffer_byte_size: usize,
    sample_count: u32,
    bound_limit: u32,
    world: World,
    camera: Camera
}

impl RayTracer {
    pub fn new(init_size: (usize, usize), event_sender: UserEventSender<UserEvent>) -> RayTracer {
        let mut new_buffer: Vec<u8> = Vec::new();
        let new_byte_size = init_size.0 * init_size.1 * 3;
        new_buffer.resize(new_byte_size, 0);

        RayTracer {
            event_sender,
            buffer: new_buffer,
            buffer_size: (init_size.0, init_size.1),
            buffer_byte_size: new_byte_size,
            sample_count: 10,
            bound_limit: 5,
            world: World::new_default(),
            camera: Camera::new_default()
        }
    }

    pub fn resize(&mut self, new_size: (usize, usize)) {
        let new_byte_size = new_size.0 * new_size.1 * 3;
        self.buffer.resize(new_byte_size, 0);
        for i in 0 .. self.buffer.len() {
            self.buffer[i] = 0;
        }

        self.buffer_byte_size = new_byte_size;
        self.buffer_size = new_size;
    } 

    pub fn run(&mut self) {
        // World
        let main_sphere = Box::new(
            Sphere::new(
                Point3::new(0.0, 0.0, -1.0), 
                0.5)
            );
        let floor_sphere = Box::new(
            Sphere::new(
                Point3::new(0.0, -100.5, -1.0), 
                100.0)
            );
        self.world.add_object(main_sphere);
        self.world.add_object(floor_sphere);

        // Camera
        let aspect_ratio = self.buffer_size.0 as f64 / self.buffer_size.1 as f64;
        self.camera.update(aspect_ratio);
    
        let total_count = self.buffer_size.0 * self.buffer_size.1;
        for y in 0 .. self.buffer_size.1 {
            for x in 0 .. self.buffer_size.0 {
                let final_color = self.multisample_ray((x, y), self.sample_count);
                self.set_buffer((x, y), &final_color, true);
            }

            let percentage = ((self.buffer_size.0) * (y + 1) * 100) / total_count;
            self.print_message(&format!("Progress... {}%", percentage));
        }

        self.print_message("Finish Raytrace!")
    }

    pub fn set_buffer(&mut self, pos: (usize, usize), color: &Color, flip_y: bool) {
        let cur_y = match flip_y {
            true => { self.buffer_size.1 - pos.1 - 1 }
            false => { pos.1 }
        };

        let index: usize = (self.buffer_size.0 * cur_y * 3 + pos.0 * 3) as usize;
        self.buffer[index + 0] = (color[0] * 256.0) as u8;
        self.buffer[index + 1] = (color[1] * 256.0) as u8;
        self.buffer[index + 2] = (color[2] * 256.0) as u8;
    }

    pub fn get_buffer(&self) -> &Vec<u8> {
        &self.buffer
    }

    pub fn get_size(&self) -> &(usize, usize) {
        &self.buffer_size
    }

    fn print_message(&self, message: &str) {
        let event = UserEvent::SetHeader(message.to_string());
        let result = self.event_sender.send_event(event);

        match result {
            _ => {}
        }
    }

    fn ray_color(&self, ray: &Ray) -> Color {
        self.reflect_ray_recursive(ray, self.bound_limit)
    }

    fn reflect_ray_recursive(&self, ray: &Ray, bound_count: u32) -> Color {
        if bound_count == 0 {
            return Color::new_default();
        }

        let hit_record = self.world.world_hit(ray, f64::EPSILON, f64::MAX);
        match hit_record {
            Ok(record) => {
                let target = record.point + Vec3::rand_in_hemisphere(&record.normal);
                let next_ray = Ray::new(record.point, target - record.point);
                self.reflect_ray_recursive(&next_ray, bound_count - 1) * 0.5
            }
            Err(()) => {
                let unit_direction = ray.get_direction().get_normal();
                let weight = 0.5 * (unit_direction.y + 1.0);
                let result = 
                    Color::new(1.0, 1.0, 1.0) * (1.0 - weight) 
                    + Color::new(0.5, 0.7, 1.0) * weight;
                result
            }
        }
    }

    fn multisample_ray(&self, screen_pos: (usize, usize), sample_count: u32) -> Color {
        let mut accmulated_color = Color::new_default();
        for _ in 0 .. sample_count {
            let u_rand = rand::thread_rng().gen_range(0.0 .. 1.0);
            let u = (screen_pos.0 as f64 + u_rand) / (self.buffer_size.0 - 1) as f64;
            let v_rand = rand::thread_rng().gen_range(0.0 .. 1.0);
            let v = (screen_pos.1 as f64 + v_rand) / (self.buffer_size.1 - 1) as f64;

            let ray = self.camera.get_ray(u, v);
            accmulated_color += self.ray_color(&ray);
        }

        let scale = 1.0 / sample_count as f64;
        for i in 0 .. 3 {
            let mut channel = accmulated_color[i];
            channel *= scale;
            channel = channel.sqrt();
            channel = channel.clamp(0.0, 1.0);
            accmulated_color.set_from_index(i, channel);
        }

        accmulated_color
    }
}

