
mod math;
mod raytracer;
mod ray;
mod object;
mod world;
mod camera;

use speedy2d::dimen::Vector2;
use speedy2d::image::{ImageSmoothingMode, ImageDataType};
use speedy2d::{Graphics2D, Window};
use speedy2d::window::{WindowHandler, WindowHelper, VirtualKeyCode, WindowCreationOptions, WindowSize, WindowPosition};

use raytracer::RayTracer;

fn main() {
    let title = "raytracer-rs";
    let width: usize = 800;
    let height: usize = 600;

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

struct RTWindowHandler {
    raytracer: RayTracer,
    title: String,
    need_raytrace: bool
}

impl RTWindowHandler {
    fn new(raytracer: RayTracer, title: &str) -> RTWindowHandler {
        RTWindowHandler { 
            raytracer: raytracer,
            title: title.to_string(),
            need_raytrace: false
         }
    }

    fn redraw_for_rt(&mut self, helper: &mut WindowHelper<UserEvent>) {
        self.need_raytrace = true;
        helper.request_redraw();
    }
}

impl WindowHandler<UserEvent> for RTWindowHandler {    
    fn on_start(&mut self, helper: &mut WindowHelper<UserEvent>, info: speedy2d::window::WindowStartupInfo) {
        self.redraw_for_rt(helper);
    }

    fn on_resize(&mut self, helper: &mut WindowHelper<UserEvent>, size_pixels: speedy2d::dimen::Vector2<u32>) {
        self.raytracer.resize((size_pixels.x as usize, size_pixels.y as usize));
        self.redraw_for_rt(helper);
    }

    fn on_draw(&mut self, helper: &mut WindowHelper<UserEvent>, graphics: &mut Graphics2D)
    {
        if self.need_raytrace == true {
            self.raytracer.run();
            self.need_raytrace = false;
        }

        let image_result = graphics.create_image_from_raw_pixels(
            ImageDataType::RGB, 
            ImageSmoothingMode::Linear,
            Vector2::new(self.raytracer.get_size().0 as u32, self.raytracer.get_size().1 as u32),
            self.raytracer.get_buffer());

        match image_result {
            Ok(image) => {
                graphics.draw_image(Vector2::new(0.0, 0.0), &image);
            }
            Err(error) => {
                print!("{}", error.error().to_string());
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
            _ => {}
        }
    }
}
