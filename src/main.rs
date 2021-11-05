
mod vec3;

use vec3::*;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0 .. image_height).rev() {
        println!("Scanline remaining: {}", j);

        for i in 1 .. image_width {
            let pixel_color = Color::new(
                i as f64 / (image_width - 1) as f64, 
                j as f64 / (image_height - 1) as f64, 
                0.25);
            Color::write_color(&pixel_color);
        }
    }
    //let testVec = Vec3::new();
    //println!("Test: {}", testVec)
}
