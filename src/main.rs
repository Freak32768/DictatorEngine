extern crate sdl2;
extern crate serde;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use serde_json::{Result, Value};
use std::path::Path;
use std::time::Duration;

static WINDOW_WIDTH: u32 = 800;
static WINDOW_HEIGHT: u32 = 600;

pub fn main() {
    let ipagp = Path::new("./ipagp.ttf");
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let window = video_subsystem
        .window("Dictator Engine", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut font_mes = ttf_context.load_font(ipagp, 32).unwrap();
    font_mes.set_style(sdl2::ttf::FontStyle::NORMAL);
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        show_text_area(
            &mut canvas,
            &mut font_mes,
            &[
                "TESTING english",
                "hello, world",
                "third line",
                "fourth line",
            ],
        );
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

// draw text in text area
fn show_text_area(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    font: &mut sdl2::ttf::Font,
    _mes: &[&str],
) {
    let texture_creator = canvas.texture_creator();
    // generate textures
    let textures: Vec<_> = _mes
        .iter()
        .map(|message| {
            let surface = font
                .render(message)
                .blended_wrapped(Color::RGB(255, 255, 255), 760)
                .unwrap();
            texture_creator
                .create_texture_from_surface(&surface)
                .unwrap()
        })
        .collect();
    // draw text area
    canvas.set_draw_color(Color::RGB(64, 64, 64));
    canvas.fill_rect(Rect::new(10, 400, 780, 190)).unwrap();
    // draw each text
    let mut y = 410;
    for texture in textures {
        let dest = Rect::new(20, y, texture.query().width, texture.query().height);
        canvas.copy(&texture, None, dest).unwrap();
        y += 40;
    }
}
