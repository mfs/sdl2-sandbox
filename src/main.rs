extern crate sdl2;
extern crate sdl2_ttf;
extern crate sdl2_image;
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
use sdl2_image::LoadTexture;

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
        .accelerated().present_vsync()
        .build().unwrap();

    let ttf_context = sdl2_ttf::init().unwrap();
    let font = ttf_context.load_font(Path::new("assets/Inconsolata-Bold.ttf"), 16).unwrap();

    let ship = renderer.load_texture(Path::new("assets/fighterspr1-small.png")).unwrap();
    let asteroid = renderer.load_texture(Path::new("assets/asteroid1.png")).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut vx = 0.0;
    let mut vy = 0.0;
    let mut rot = 0.0;

    let mut timer = sdl_context.timer().unwrap();
    let mut step = timer.ticks() as f64 / 1000.0;

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

        let tmp = timer.ticks() as f64 / 1000.0;
        rot += 15.0 * (tmp - step);
        if rot > 360.0 {
            rot -= 360.0;
        }
        vy -= 200.0 * (tmp - step);
        step = tmp;

        // Render a fully black window
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        let stars = get_stars(vx, vy, 800.0, 600.0);

        // track starfield extents for debuging
        let mut stars_x_min: f64 = 0.0;
        let mut stars_x_max: f64 = 0.0;
        let mut stars_y_min: f64 = 0.0;
        let mut stars_y_max: f64 = 0.0;
        let mut stars_n: u64 = 0;
        let mut stars_rendered_n: u64 = 0;

        for star in stars {
            stars_n += 1;
            let (x, y, brightness) = star;
            stars_x_min = stars_x_min.min(x);
            stars_x_max = stars_x_max.max(x);
            stars_y_min = stars_y_min.min(y);
            stars_y_max = stars_y_max.max(y);
            if x - vx < 0.0 || x - vx >= 800.0 || y - vy < 0.0 || y - vy >= 600.0 {
                continue;
            }
            stars_rendered_n += 1;
            let c = (brightness * 255.0).round() as u8;
            renderer.set_draw_color(Color::RGB(c, c, c));
            renderer.draw_point(
                Point::new((x - vx).round() as i32, (y - vy).round() as i32)
            ).unwrap();
        }
        renderer.copy_ex(&ship, None, Some(Rect::new(351, 215, 98, 169)), rot, None, false, false ).unwrap();
        renderer.copy(&asteroid, Some(Rect::new(0, 0, 72, 72)), Some(Rect::new(60, 60, 72, 72)));
        text(&mut renderer, &font, &format!("Starfield x: {:.2} y: {:.2} r: {:.2} n: {} n-ren: {}", vx, vy, rot, stars_n, stars_rendered_n),  10, 10);
        text(&mut renderer, &font, &format!("Extents x: {} {} y: {} {}", stars_x_min, stars_x_max, stars_y_min, stars_y_max),  10, 30);
        renderer.present();
    }

}
