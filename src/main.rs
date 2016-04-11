extern crate sdl2;
extern crate sdl2_ttf;
extern crate fnv;

#[macro_use]
extern crate cfor;

mod starfield;
use starfield::get_stars;

use std::path::Path;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::render::{Renderer, TextureQuery};

fn text(renderer: &mut Renderer, font: &sdl2_ttf::Font, text: &str, x: i32, y: i32) {
    let surface = font.render(text)
                .blended(Color::RGBA(255, 255, 255, 255)).unwrap();
    let mut texture = renderer.create_texture_from_surface(&surface).unwrap();
    let TextureQuery { width, height, .. } = texture.query();

    renderer.copy(&mut texture, None, Some(Rect::new(x, y, width, height)));
}

fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    // Create the window
    let window = video.window("Starfield", 800, 600)
        .position_centered().opengl()
        .build().unwrap();

    let mut renderer = window.renderer()
        .accelerated()
        .build().unwrap();

    let ttf_context = sdl2_ttf::init().unwrap();
    let font = ttf_context.load_font(Path::new("assets/Inconsolata-Bold.ttf"), 16).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut vx = 0.0;
    let mut vy = 0.0;

    // track starfield extents for debuging
    let mut stars_x_min: f64 = 0.0;
    let mut stars_x_max: f64 = 0.0;
    let mut stars_y_min: f64 = 0.0;
    let mut stars_y_max: f64 = 0.0;

    // main loop
    'main: loop {

        for event in event_pump.poll_iter() {
            use sdl2::keyboard::Keycode::*;
            match event {
                Event::KeyDown { keycode, .. } => {
                    match keycode {
                        Some(Left)  => vx -= 1.0,
                        Some(Right) => vx += 1.0,
                        Some(Down)  => vy += 1.0,
                        Some(Up)    => vy -= 1.0,
                        _           => break 'main,
                    }
                },
                _ => {}
            }
        }

        // Render a fully black window
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        let stars = get_stars(vx, vy, 800.0, 600.0);

        for star in stars {
            let (x, y, brightness) = star;
            stars_x_min = stars_x_min.min(x);
            stars_x_max = stars_x_max.max(x);
            stars_y_min = stars_y_min.min(y);
            stars_y_max = stars_y_max.max(y);
            let c = (brightness * 255.0).round() as u8;
            renderer.set_draw_color(Color::RGB(c, c, c));
            renderer.draw_point(
                Point::new((x - vx).round() as i32, (y - vy).round() as i32)
            ).unwrap();
        }
        text(&mut renderer, &font, &format!("Starfield x: {} y: {}", vx, vy),  10, 10);
        text(&mut renderer, &font, &format!("Extents x: {} {} y: {} {}", stars_x_min, stars_x_max, stars_y_min, stars_y_max),  10, 30);
        renderer.present();
    }

}
