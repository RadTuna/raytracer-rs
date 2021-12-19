
pub mod math;
mod raytracer;
mod ray;

/*
fn main() {
    raytracer::raytracer_run();
}
*/

use speedy2d::dimen::Vector2;
use speedy2d::image::{ImageSmoothingMode, ImageDataType};
use speedy2d::{Graphics2D, Window};
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::window::VirtualKeyCode;

use raytracer::RayTracer;

fn main() {
    let width: usize = 800;
    let height: usize = 600;

    let window = Window::new_centered("raytracer-rs", (width as u32, height as u32)).unwrap();
    let raytracer = RayTracer::new((width, height));
    window.run_loop(RTWindowHandler{ raytracer });
}

struct RTWindowHandler {
    raytracer: RayTracer
}

impl WindowHandler for RTWindowHandler
{
    fn on_start(&mut self, helper: &mut WindowHelper<()>, info: speedy2d::window::WindowStartupInfo) {
        self.raytracer.run();
    }

    fn on_resize(&mut self, helper: &mut WindowHelper<()>, size_pixels: speedy2d::dimen::Vector2<u32>) {

    }

    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
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

    fn on_key_down(&mut self, helper: &mut WindowHelper<()>, virtual_key_code: Option<speedy2d::window::VirtualKeyCode>, scancode: speedy2d::window::KeyScancode) {
        if let Some(virtual_code) = virtual_key_code {
            match virtual_code {
                VirtualKeyCode::R => {
                    helper.request_redraw();
                }
                _ => {}
            }
        }
    }
}
