/*
   Copyright 2021 Freak32768

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
o   See the License for the specific language governing permissions and
   limitations under the License.
*/
extern crate sdl2;
extern crate serde;

use dictator_engine::dictator_util;
use sdl2::pixels::Color;
use std::env;
use std::path::Path;

static WINDOW_WIDTH: u32 = 800;
static WINDOW_HEIGHT: u32 = 600;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Error: wrong arguments");
        println!("$ dictator_engine [json file]");
    }
    let ipam = Path::new("./dictator_example/ipam.ttf");
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
    let root_scene = dictator_util::new_scene(Path::new("./dictator_example/root_scene.json"));
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut next = "root".to_string();
    canvas.clear();
    while next != "__EXIT__" {
        next = dictator_util::show_scene(
            &root_scene,
            &next,
            &mut event_pump,
            &mut canvas,
            &ipa_mincho,
        );
    }
}
