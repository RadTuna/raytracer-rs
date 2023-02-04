
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};

use crate::UserEvent;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::material::dielectric::Dielectric;
use crate::math::vec3::{Color, Point3};
use crate::threading::RayWorkerManager;
use crate::threading::ray_worker::{RayWorkerSettings, RayResult};
use crate::world::World;
use crate::object::sphere::Sphere;
use crate::camera::{Camera, CameraSettings};

use rand::{thread_rng, Rng};
use speedy2d::window::UserEventSender;


#[derive(Clone, Copy)]
pub enum RayTracerState {
    Idle,
    Working
}

#[derive(Clone, Copy)]
pub struct RayTracerSettings {
    sample_count: u32,
    bound_limit: u32,
    receive_limit: u32
}

#[derive(Clone)]
pub struct RayTracerBuffer {
    buffer: Vec<u8>,
    buffer_size: (usize, usize),
    buffer_byte_size: usize
}

impl RayTracerBuffer {
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

    pub fn resize(&mut self, new_size: (usize, usize)) {
        let new_byte_size = new_size.0 * new_size.1 * 3;
        self.buffer.resize(new_byte_size, 0);
        for i in 0 .. self.buffer.len() {
            self.buffer[i] = 0;
        }

        self.buffer_byte_size = new_byte_size;
        self.buffer_size = new_size;
    }

    pub fn get_buffer(&self) -> &Vec<u8> {
        &self.buffer
    }

    pub fn get_buffer_size(&self) -> (usize, usize) {
        self.buffer_size
    }

    pub fn get_byte_size(&self) -> usize {
        self.buffer_byte_size
    }
}


pub struct RayTracer {
    // systems
    event_sender: UserEventSender<UserEvent>,
    ray_worker_manager: RayWorkerManager,
    buffer_sender: Sender<RayResult>,
    buffer_receiver: Receiver<RayResult>,
    core_thread_nums: usize,

    // raw datas
    buffer: RayTracerBuffer,

    // scene
    world: World,
    camera: Camera,
    settings: RayTracerSettings,

    // state
    state: RayTracerState,
    received_packet: usize,
    buffer_updated: bool
}

impl RayTracer {
    pub fn new(init_size: (usize, usize), core_thread_nums: usize, event_sender: UserEventSender<UserEvent>) -> RayTracer {
        let mut new_buffer: Vec<u8> = Vec::new();
        let new_byte_size = init_size.0 * init_size.1 * 3;
        new_buffer.resize(new_byte_size, 0);
        
        let raytracer_buffer = RayTracerBuffer {
            buffer: new_buffer,
            buffer_size: init_size,
            buffer_byte_size: new_byte_size
        };

        let ray_tracer_settings = RayTracerSettings {
            sample_count: 1000,
            bound_limit: 100,
            receive_limit: 10
        };

        let (sender, receiver): (Sender<RayResult>, Receiver<RayResult>) = channel();

        RayTracer {
            event_sender,
            ray_worker_manager: RayWorkerManager::new(),
            buffer_sender: sender,
            buffer_receiver: receiver,
            core_thread_nums,
            buffer: raytracer_buffer,
            world: World::new_default(),
            camera: Camera::new_default(),
            settings: ray_tracer_settings,
            state: RayTracerState::Idle,
            received_packet: 0,
            buffer_updated: false
        }
    }

    pub fn resize(&mut self, new_size: (usize, usize)) {
        self.buffer.resize(new_size);
    } 

    pub fn run(&mut self) {
        if let RayTracerState::Idle = self.state {} else {
            return;
        }

        self.state = RayTracerState::Working;

        self.build_world();
        self.update_camera();

        let mut ray_worker_settings = RayWorkerSettings {
            screen_size: self.get_buffer_size(),
            bound_y: (0, 0),
            sample_count: self.settings.sample_count,
            bound_limit: self.settings.bound_limit,
        };

        let cpu_nums = num_cpus::get();
        let worker_nums = if cpu_nums > self.core_thread_nums { cpu_nums - self.core_thread_nums } else { 1 };
        let per_worker_y = self.get_buffer_size().1 / worker_nums;
        let mut prev_max_bound: usize = 0;
        for i in 0 .. worker_nums {
            ray_worker_settings.bound_y.0 = prev_max_bound;
            ray_worker_settings.bound_y.1 += per_worker_y;
            prev_max_bound = ray_worker_settings.bound_y.1;
            if i == (worker_nums - 1) {
                ray_worker_settings.bound_y.1 = self.get_buffer_size().1;
            }

            let copied_world = self.world.clone();
            let copied_camera = self.camera.clone();
            let copied_sender = self.buffer_sender.clone();
            self.ray_worker_manager.start_worker(copied_world, copied_camera, copied_sender, ray_worker_settings);
        }
    }

    pub fn tick(&mut self) {
        if let RayTracerState::Working = self.state {} else {
            return;
        }

        let worker_nums = self.ray_worker_manager.get_worker_nums();
        let expected_packet = worker_nums * self.settings.sample_count as usize;
        if self.received_packet >= expected_packet {
            self.print_message("finished raytracing!", false);
            self.ray_worker_manager.join_workers();
            self.state = RayTracerState::Idle;
            self.received_packet = 0;
            return;
        }

        let mut receive_count = 0;
        while receive_count < self.settings.receive_limit {
            match self.buffer_receiver.recv() {
                Ok(ray_result) => {
                    self.received_packet += 1;
                    self.buffer_updated = true;
                    self.apply_worker_buffer(ray_result);
                }
                Err(_) => { 
                    break;
                }
            }

            receive_count += 1;
        }

        let progress_percentage = (self.received_packet as f64 / expected_packet as f64) * 100.0;
        let message = format!("progress: {percentage:.2}%", percentage = progress_percentage);
        self.print_message(&message, true);
    }

    pub fn get_raytracer_state(&self) -> RayTracerState {
        self.state
    }

    pub fn is_buffer_updated(&self) -> bool {
        self.buffer_updated
    }

    pub fn consume_buffer(&mut self) -> &RayTracerBuffer {
        self.buffer_updated = false;
        &self.buffer
    }

    pub fn get_buffer_size(&self) -> (usize, usize) {
        self.buffer.get_buffer_size()
    }

    pub fn print_message(&self, message: &str, ignore_print: bool) {
        if !ignore_print {
            println!("{msg}", msg = message);
        }

        let event = UserEvent::SetHeader(message.to_string());
        let result = self.event_sender.send_event(event);
        match result {
            _ => {}
        }
    }

    fn apply_worker_buffer(&mut self, ray_result: RayResult) {
        for x in 0 .. self.get_buffer_size().0 {
            for y in ray_result.bound_y.0 .. ray_result.bound_y.1 {
                let offseted_y = y - ray_result.bound_y.0;
                let target_index = self.get_buffer_size().0 * offseted_y + x;
                let target_color = ray_result.srgb_buffer[target_index];
                self.buffer.set_buffer((x, y), &target_color, true);
            }
        }
    }

    fn build_world(&mut self) {
        // ground
        let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
        let ground_mesh = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(ground_material));
        self.world.add_object(Box::new(ground_mesh));
    
        // random small spheres
        for a in -15 .. 15 {
            for b in -15 .. 15 {
                let mut rng = thread_rng();
                let center = Point3::new(
                    a as f64 + 0.9 * rng.gen_range(0.0 .. 1.0), 
                    0.2,
                    b as f64 + 0.9 * rng.gen_range(0.0 .. 1.0));

                let can_spawn = (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9;

                if can_spawn {
                    let choose_material = rng.gen_range(0.0 .. 1.0);
                    let mesh = if choose_material < 0.5 { // diffuse
                        let albedo = Color::rand() * Color::rand();
                        let material = Lambertian::new(albedo);
                        Sphere::new(center, 0.2, Box::new(material))
                    } else if choose_material < 0.8 { // metal
                        let albedo = Color::rand_range((0.5, 1.0));
                        let fuzziness = rng.gen_range(0.0 .. 0.1);
                        let material = Metal::new(albedo, fuzziness);
                        Sphere::new(center, 0.2, Box::new(material))
                    } else { // glass
                        let material = Dielectric::new(1.5);
                        Sphere::new(center, 0.2, Box::new(material))
                    };

                    self.world.add_object(Box::new(mesh));
                };
            }

            // big sphere
            let center_material = Dielectric::new(1.5);
            let center_mesh = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, Box::new(center_material));
            self.world.add_object(Box::new(center_mesh));

            let back_material = Lambertian::new(Color::new(0.4, 0.2, 0.1));
            let back_mesh = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, Box::new(back_material));
            self.world.add_object(Box::new(back_mesh));

            let front_material = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
            let front_mesh = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, Box::new(front_material));
            self.world.add_object(Box::new(front_mesh));
        }
    }

    fn update_camera(&mut self) {
        let look_from = Point3::new(13.0, 2.0, 3.0);
        let look_to = Point3::new(0.0, 0.0, 0.0);
        let aspect_ratio = self.get_buffer_size().0 as f64 / self.get_buffer_size().1 as f64;

        let field_of_view = 20.0;
        let aperture = 0.1;
        let dist_to_focus = 10.0;
        let settings = CameraSettings::new(field_of_view, aperture, dist_to_focus);

        self.camera.update(look_from, look_to, aspect_ratio, settings);
    }

}

