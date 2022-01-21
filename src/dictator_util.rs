/*
   Copyright 2021 Freak32768

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/
extern crate sdl2;
extern crate serde;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Duration;

pub fn new_scene(filepath: &Path) -> Value {
    let mut file = match File::open(filepath) {
        Err(_) => panic!("could not open file {}", filepath.display(),),
        Ok(file) => file,
    };
    let mut json_str = String::new();
    match file.read_to_string(&mut json_str) {
        Err(_) => panic!("could not read {}", filepath.display()),
        Ok(_) => 0,
    };
    let val: Value = serde_json::from_str(&json_str).unwrap();
    val
}
pub fn show_scene(
    scene: &Value,
    scene_name: &str,
    event_pump: &mut sdl2::EventPump,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    font: &sdl2::ttf::Font,
) -> String {
    let _s = scene
        .as_object()
        .unwrap()
        .get(scene_name)
        .unwrap()
        .as_object()
        .unwrap();
    let _texts = _s.get("texts").unwrap().as_array().unwrap();
    let mut texts: Vec<String> = vec![];
    for text in _texts {
        texts.push(text.as_str().unwrap().to_string());
    }
    let line_max = 5;
    let mut cnt = 0;
    let tex_creator = canvas.texture_creator();
    let mut texts_vec: Vec<Vec<String>> = vec![];
    'splitter: loop {
        if cnt + line_max < texts.len() {
            texts_vec.push(texts[cnt..cnt + line_max].to_vec());
            cnt += line_max;
        } else {
            texts_vec.push(texts[cnt..].to_vec());
            break 'splitter;
        }
    }
    let picture_path = _s.get("picture_path").unwrap().as_str().unwrap();
    let picture = tex_creator.load_texture(Path::new(picture_path)).unwrap();
    for text_vec in texts_vec {
        canvas.clear();
        canvas.copy(&picture, None, None).unwrap();
        show_text_area(text_vec, canvas, &font);
        canvas.present();
        match wait(event_pump) {
            Keycode::Escape => return "__EXIT__".to_string(),
            Keycode::Return => {}
            _ => {}
        }
    }
    let _r = _s.get("next").unwrap().as_array().unwrap();
    let routes: Vec<&str> = _r.iter().map(|r| r.as_str().unwrap()).collect();
    let next = next(event_pump, routes.len().try_into().unwrap());
    if next == -1 {
        "__EXIT__".to_string()
    } else {
        routes[<usize as std::convert::TryFrom<i8>>::try_from(next).unwrap()].to_string()
    }
}

pub fn show_text_area(
    texts: Vec<String>,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    font: &sdl2::ttf::Font,
) {
    let texture_creator = canvas.texture_creator();
    // generate textures
    let mut textures: Vec<sdl2::render::Texture> = vec![];
    for text in texts {
        let surface = font
            .render(&text)
            .blended_wrapped(Color::RGB(255, 255, 255), 760)
            .unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        textures.push(texture);
    }
    // draw text area
    canvas.set_draw_color(Color::RGB(64, 64, 64));
    canvas.fill_rect(Rect::new(10, 400, 780, 190)).unwrap();
    // draw each text
    let mut y = 410;
    for texture in textures {
        let dest = Rect::new(20, y, texture.query().width, texture.query().height);
        canvas.copy(&texture, None, dest).unwrap();
        y += 30;
    }
}
pub fn wait(event_pump: &mut sdl2::EventPump) -> Keycode {
    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return Keycode::Escape,
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => return Keycode::Return,
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
pub fn next(event_pump: &mut sdl2::EventPump, routes: u8) -> i8 {
    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return -1,
                Event::KeyDown {
                    keycode: Some(Keycode::Num1),
                    ..
                } => {
                    if routes >= 1 {
                        return 0;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num2),
                    ..
                } => {
                    if routes >= 2 {
                        return 1;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num3),
                    ..
                } => {
                    if routes >= 3 {
                        return 2;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num4),
                    ..
                } => {
                    if routes >= 4 {
                        return 3;
                    }
                }
                _ => {}
            }
        }
    }
}
