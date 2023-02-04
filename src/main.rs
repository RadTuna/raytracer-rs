#![allow(dead_code)]
#![allow(unused_variables)]

mod math;
mod raytracer;
mod ray;
mod object;
mod world;
mod camera;
mod material;
mod threading;


use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use speedy2d::dimen::Vector2;
use speedy2d::image::{ImageSmoothingMode, ImageDataType};
use speedy2d::{Graphics2D, Window};
use speedy2d::window::{WindowHandler, WindowHelper, VirtualKeyCode, WindowCreationOptions, WindowSize, WindowPosition};

use raytracer::{RayTracer, RayTracerBuffer};
use raytracer::RayTracerState;


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

    let (command_sender, command_receiver): (Sender<RayTracerCommand>, Receiver<RayTracerCommand>) = channel();
    let (buffer_sender, buffer_receiver): (Sender<RayTracerBuffer>, Receiver<RayTracerBuffer>) = channel();

    let thread_handle = thread::spawn(move || {
        let raytracer = RayTracer::new((width, height), 2, event_sender);
        raytracer_main(raytracer, command_receiver, buffer_sender);
    });


    window.run_loop(RTWindowHandler::new(title, command_sender, buffer_receiver));
}


enum RayTracerCommand {
    Run,
    Exit
}

fn raytracer_main(mut raytracer: RayTracer, command_receiver: Receiver<RayTracerCommand>, buffer_sender: Sender<RayTracerBuffer>) {
    loop {
        match command_receiver.try_recv() {
            Ok(command) => {
                match command {
                    RayTracerCommand::Run => {
                        if let RayTracerState::Idle = raytracer.get_raytracer_state() {
                            raytracer.run();
                        }
                    }
                    RayTracerCommand::Exit => {
                        break;
                    }
                }
            }
            _ => { }
        }

        if let RayTracerState::Working = raytracer.get_raytracer_state() {
            raytracer.tick();
            let copied_buffer = raytracer.consume_buffer().clone();
            buffer_sender.send(copied_buffer).unwrap();
        }
    }
}


pub enum UserEvent {
    SetHeader(String)
}

struct RTWindowHandler {
    title: String,
    command_sender: Sender<RayTracerCommand>,
    buffer_receiver: Receiver<RayTracerBuffer>
}

impl RTWindowHandler {
    fn new(title: &str, command_sender: Sender<RayTracerCommand>, buffer_receiver: Receiver<RayTracerBuffer>) -> RTWindowHandler {
        RTWindowHandler { 
            title: title.to_string(),
            command_sender,
            buffer_receiver
         }
    }

    fn redraw_for_rt(&mut self, helper: &mut WindowHelper<UserEvent>) {
        self.command_sender.send(RayTracerCommand::Run).unwrap();
        helper.request_redraw();
    }
}

impl WindowHandler<UserEvent> for RTWindowHandler {    
    fn on_start(&mut self, helper: &mut WindowHelper<UserEvent>, info: speedy2d::window::WindowStartupInfo) {
        self.redraw_for_rt(helper);
    }

    fn on_draw(&mut self, helper: &mut WindowHelper<UserEvent>, graphics: &mut Graphics2D) {
        match self.buffer_receiver.try_recv() {
            Ok(buffer) => {
                let buffer_size = buffer.get_buffer_size();
                let image_result = graphics.create_image_from_raw_pixels(
                    ImageDataType::RGB, 
                    ImageSmoothingMode::Linear,
                    Vector2::new(buffer_size.0 as u32, buffer_size.1 as u32),
                    &buffer.get_buffer());

                match image_result {
                    Ok(image) => {
                        graphics.draw_image(Vector2::new(0.0, 0.0), &image);
                    }
                    Err(error) => {
                        print!("{}", error.error().to_string());
                    }
                }
            }
            _ => { }
        }

        helper.request_redraw();
    }

    fn on_key_down(&mut self, helper: &mut WindowHelper<UserEvent>, virtual_key_code: Option<speedy2d::window::VirtualKeyCode>, scancode: speedy2d::window::KeyScancode) {
        if let Some(virtual_code) = virtual_key_code {
            match virtual_code {
                VirtualKeyCode::R => {
                    self.redraw_for_rt(helper);
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
