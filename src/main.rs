use image::{RgbImage, ImageBuffer};
use show_image::{create_window, ImageInfo, ImageView, event};

mod noise_handler;

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    const WIDTH: u32 = 2000;
    const HEIGHT: u32 = 2000;

    let mut image: RgbImage = ImageBuffer::from_fn(WIDTH, HEIGHT, |_, _| image::Rgb([225, 0, 0]));

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let red_channel = (noise_handler::generate_perlin_noise(x as f64, y as f64, 0.07, 5) * 128.0) as u8;
            let green_channel = (noise_handler::generate_perlin_noise(x as f64 + 100.0, y as f64 + 100.0, 0.07, 5) * 128.0) as u8;
            let blue_channel = (noise_handler::generate_perlin_noise(x as f64 + 200.0, y as f64 + 200.0, 0.07, 5) * 128.0) as u8;

            let pixel = image.get_pixel_mut(x, y);
            *pixel = image::Rgb([red_channel, green_channel, blue_channel]);
        }
    }

    image.save("noise.png")?;

    let view = ImageView::new(ImageInfo::rgb8(WIDTH, HEIGHT), &image);
    let window = create_window("Noise", Default::default())?;
    window.set_image("noise_pattern", view)?;

    for event in window.event_channel()? {
        if let event::WindowEvent::KeyboardInput(key_event) = event {
            if let Some(event::VirtualKeyCode::Escape) = key_event.input.key_code {
                if key_event.input.state.is_pressed() {
                    break;
                }
            }
        }
    }

    Ok(())
}
