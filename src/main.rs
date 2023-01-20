#![allow(dead_code)]
#![allow(unused_variables)]

mod math;
mod raytracer;
mod ray;
mod object;
mod world;
mod camera;
mod material;

use speedy2d::dimen::Vector2;
use speedy2d::image::{ImageSmoothingMode, ImageDataType};
use speedy2d::{Graphics2D, Window};
use speedy2d::window::{WindowHandler, WindowHelper, VirtualKeyCode, WindowCreationOptions, WindowSize, WindowPosition};

use raytracer::RayTracer;

use std::thread;
use std::sync::mpsc;


fn main() {
    let title = "raytracer-rs";
    let width: usize = 1280;
    let height: usize = 720;

    let mut creation_options = WindowCreationOptions::new_windowed(
        WindowSize::PhysicalPixels(Vector2::new(width as u32, height as u32)),
        Some(WindowPosition::Center)
    );
    creation_options = creation_options.with_resizable(false);

    let window = Window::<UserEvent>::new_with_user_events(title, creation_options).unwrap();
    let event_sender = window.create_user_event_sender();
    let raytracer = RayTracer::new((width, height), event_sender);

    window.run_loop(RTWindowHandler::new(raytracer, title));
}

pub enum UserEvent {
    SetHeader(String)
}

struct RTReturn {
    raytracer: RayTracer,
    buffer: Vec<u8>,
    size: (usize, usize)
}

struct RTWindowHandler {
    raytracer: Option<RayTracer>,
    title: String,
    need_raytrace: bool,
    receiver: Option<mpsc::Receiver<RTReturn>>
}

impl RTWindowHandler {
    fn new(raytracer: RayTracer, title: &str) -> RTWindowHandler {
        RTWindowHandler { 
            raytracer: Option::Some(raytracer),
            title: title.to_string(),
            need_raytrace: false,
            receiver: Option::None
         }
    }

    fn redraw_for_rt(&mut self, helper: &mut WindowHelper<UserEvent>) {
        self.need_raytrace = true;
        helper.request_redraw();
    }

    fn run_async_raytrace(&mut self) {
        let (tx, rx) = mpsc::channel();
        self.receiver = Option::Some(rx);

        let local_raytracer = self.raytracer.take();
        thread::spawn(move || {
            if let Some(mut real_raytracer) = local_raytracer
            {
                real_raytracer.run();
                let buffer = real_raytracer.get_buffer().clone();
                let size = real_raytracer.get_size().clone();
                let result = RTReturn { 
                    raytracer: real_raytracer, 
                    buffer, 
                    size 
                };

                tx.send(result).unwrap();
            }
        });
        self.need_raytrace = false;
    }
}

impl WindowHandler<UserEvent> for RTWindowHandler {    
    fn on_start(&mut self, helper: &mut WindowHelper<UserEvent>, info: speedy2d::window::WindowStartupInfo) {
        self.redraw_for_rt(helper);
    }

    fn on_resize(&mut self, helper: &mut WindowHelper<UserEvent>, size_pixels: speedy2d::dimen::Vector2<u32>) {
        match &mut self.raytracer {
            Some(local_raytracer) => {
                local_raytracer.resize((size_pixels.x as usize, size_pixels.y as usize));
            }
            _ => { }
        }
        self.redraw_for_rt(helper);
    }

    fn on_draw(&mut self, helper: &mut WindowHelper<UserEvent>, graphics: &mut Graphics2D)
    {
        if self.need_raytrace == true {
            self.run_async_raytrace();
        }

        if let Some(local_receiver) = &self.receiver
        {
            let result = local_receiver.try_recv();
            match result
            {
                Ok(real_result) => {
                    let image_result = graphics.create_image_from_raw_pixels(
                        ImageDataType::RGB, 
                        ImageSmoothingMode::Linear,
                        Vector2::new(real_result.size.0 as u32, real_result.size.1 as u32),
                        &real_result.buffer);
            
                    match image_result {
                        Ok(image) => {
                            graphics.draw_image(Vector2::new(0.0, 0.0), &image);
                        }
                        Err(error) => {
                            print!("{}", error.error().to_string());
                        }
                    }

                    self.raytracer = Some(real_result.raytracer);
                }
                Err(_) => { }
            }
        }

        helper.request_redraw();
    }

    fn on_key_down(&mut self, helper: &mut WindowHelper<UserEvent>, virtual_key_code: Option<speedy2d::window::VirtualKeyCode>, scancode: speedy2d::window::KeyScancode) {
        if let Some(virtual_code) = virtual_key_code {
            match virtual_code {
                VirtualKeyCode::R => {
                    helper.request_redraw();
                }
                _ => {}
            }
        }
    }

    fn on_user_event(&mut self, helper: &mut WindowHelper<UserEvent>, user_event: UserEvent) {
        match user_event {
            UserEvent::SetHeader(message) => {
                let mut new_header = self.title.clone();
                new_header.push_str(" / ");
                new_header.push_str(&message);
                helper.set_title(&new_header);
            }
        }
    }
}
