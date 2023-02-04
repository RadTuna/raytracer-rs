
use crate::camera::Camera;
use crate::world::World;
use crate::math::vec3::Color;
use crate::ray::Ray;
use std::sync::mpsc::Sender;
use rand::Rng;


#[derive(Clone, Copy)]
pub struct RayWorkerSettings {
    pub screen_size: (usize, usize),
    pub bound_y: (usize, usize),
    pub sample_count: u32,
    pub bound_limit: u32,
}

#[derive(Clone)]
pub struct RayResult {
   pub srgb_buffer: Vec<Color>,
   pub bound_y: (usize, usize) 
}

pub struct RayWorker {
    world: World,
    camera: Camera,
    accumulated_buffer: Vec<Color>,
    srgb_buffer: Vec<Color>,
    buffer_sender: Sender<RayResult>,
    settings: RayWorkerSettings
}

impl RayWorker {
    pub fn new(world: World, camera: Camera, pixel_sender: Sender<RayResult>, settings: RayWorkerSettings) -> RayWorker {
        if settings.bound_y.0 > settings.bound_y.1 {
            panic!("wrong ray worker settings!");
        }

        let y_size = settings.bound_y.1 - settings.bound_y.0;
        let buffer_size = settings.screen_size.0 * y_size;

        let mut accumulated_buffer: Vec<Color> = Vec::new();
        accumulated_buffer.resize(buffer_size, Color::new_default());

        let mut srgb_buffer: Vec<Color> = Vec::new();
        srgb_buffer.resize(buffer_size, Color::new_default());

        RayWorker { 
            world, 
            camera, 
            accumulated_buffer,
            srgb_buffer,
            buffer_sender: pixel_sender, 
            settings 
        }
    }

    pub fn run(&mut self) {
        println!("start ray worker (id: {id})", id = self.settings.bound_y.0);

        let aspect_ratio = self.settings.screen_size.0 as f64 / self.settings.screen_size.1 as f64;
        self.camera.update(aspect_ratio);

        let start_y = self.settings.bound_y.0;
        let end_y = self.settings.bound_y.1;
        for sample_count in 0 .. self.settings.sample_count {

            for y in start_y .. end_y {
                for x in 0 .. self.settings.screen_size.0 {
                    let final_color = self.sample_ray((x, y));
                    self.apply_color((x, y), &final_color, sample_count + 1);
                }
            }

            // send srgb_buffer
            let send_result = RayResult {
                srgb_buffer: self.srgb_buffer.clone(),
                bound_y: self.settings.bound_y
            };
            self.buffer_sender.send(send_result).unwrap();
        }

        println!("ended ray worker (id: {id})", id = self.settings.bound_y.0);
    }

    fn apply_color(&mut self, position: (usize, usize), color: &Color, sample_count: u32) {
        if sample_count <= 0 {
            panic!("wrong sample count param!");
        }

        // set buffer
        let offseted_y = position.1 - self.settings.bound_y.0;
        let index: usize = (self.settings.screen_size.0 * offseted_y + position.0) as usize;
        self.accumulated_buffer[index] += *color;

        // send gamma corrected color
        let mut corrected_color = self.accumulated_buffer[index];
        let scale = 1.0 / sample_count as f64;
        for i in 0 .. 3 {
            let mut channel = corrected_color[i];
            channel *= scale;
            channel = channel.sqrt();
            channel = channel.clamp(0.0, 1.0);
            corrected_color.set_from_index(i, channel);
        }

        self.srgb_buffer[index] = corrected_color;
    }

    fn ray_color(&self, ray: &Ray) -> Color {
        self.reflect_ray_recursive(ray, self.settings.bound_limit)
    }

    fn reflect_ray_recursive(&self, ray: &Ray, bound_count: u32) -> Color {
        if bound_count == 0 {
            return self.world.get_sky_color();
        }

        let hit_record = self.world.world_hit(ray, 0.0001, f64::MAX);

        let out_color: Color;
        match hit_record {
            Ok(record) => {
                let materal_result = record.material.scatter(ray, &record);
                match materal_result {
                    Some(result) => {
                        out_color = result.attenuation * self.reflect_ray_recursive(&result.scattered_ray, bound_count - 1);
                    }
                    _ => { 
                        out_color = self.world.get_sky_color();
                    }
                }
            }
            Err(()) => {
                out_color = self.world.get_sky_color();
            }
        }

        out_color
    }

    fn sample_ray(&self, screen_pos: (usize, usize)) -> Color {
        let u_rand = rand::thread_rng().gen_range(0.0 .. 1.0);
        let u = (screen_pos.0 as f64 + u_rand) / (self.settings.screen_size.0 - 1) as f64;
        let v_rand = rand::thread_rng().gen_range(0.0 .. 1.0);
        let v = (screen_pos.1 as f64 + v_rand) / (self.settings.screen_size.1 - 1) as f64;

        let ray = self.camera.get_ray(u, v);
        self.ray_color(&ray)
    }

}
