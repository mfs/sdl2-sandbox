extern crate sdl2;

#[macro_use]
extern crate cfor;

mod starfield;
use starfield::get_stars;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::event::Event;
use std::thread;
use std::time::Duration;

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

    let mut event_pump = sdl_context.event_pump().unwrap();

    // main loop
    'main: loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { .. } => break 'main,
                _ => {}
            }
        }

        // Render a fully black window
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        let stars = get_stars(0.0, 0.0, 800.0, 600.0);

        for star in stars {
            let (x, y, brightness) = star;
            let c = (brightness * 255.0).round() as u8;
            renderer.set_draw_color(Color::RGB(c, c, c));
            renderer.draw_point(Point::new(x.round() as i32, y.round() as i32)).unwrap();
        }

        renderer.present();
    }

}
