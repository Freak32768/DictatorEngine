extern crate sdl2;
extern crate serde;

use dictator_engine::dictator_util;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::env;
use std::path::Path;
use std::time::Duration;

static WINDOW_WIDTH: u32 = 800;
static WINDOW_HEIGHT: u32 = 600;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let ipam = Path::new("./ipam.ttf");
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(sdl2::image::InitFlag::PNG);
    let ttf_context = sdl2::ttf::init().unwrap();

    let window = video_subsystem
        .window("Dictator Engine", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut ipa_mincho = ttf_context.load_font(ipam, 24).unwrap();
    ipa_mincho.set_style(sdl2::ttf::FontStyle::NORMAL);
    let mut root_scene = dictator_util::Scene::new(Path::new("root_scene.json"));
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        root_scene.show_scene(&mut canvas, &ipa_mincho);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => root_scene.next(),
                _ => {}
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
