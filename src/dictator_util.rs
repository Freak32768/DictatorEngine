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
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Scene {
    title: String,
    picture_path: String,
    texts: Vec<Vec<String>>,
    text_count: usize,
}

impl Scene {
    pub fn new(filepath: &Path) -> Scene {
        let mut file = match File::open(filepath) {
            Err(_) => panic!("could not open file {}", filepath.display(),),
            Ok(file) => file,
        };
        let mut json_str = String::new();
        match file.read_to_string(&mut json_str) {
            Err(_) => panic!("could not read {}", filepath.display()),
            Ok(_) => 0,
        };
        let val: Scene = serde_json::from_str(&json_str).unwrap();
        val
    }
    pub fn show_scene(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        font: &sdl2::ttf::Font,
    ) {
        let tex_creator = canvas.texture_creator();
        let picture = tex_creator
            .load_texture(Path::new(&self.picture_path))
            .unwrap();
        canvas.copy(&picture, None, None).unwrap();
        self.show_text_area(canvas, &font);
    }
    pub fn show_text_area(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        font: &sdl2::ttf::Font,
    ) {
        let texture_creator = canvas.texture_creator();
        // generate textures
        let textures: Vec<_> = self.texts[self.text_count]
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
            y += 30;
        }
    }
    pub fn next(&mut self) {
        if self.texts.len() > self.text_count + 1 {
            self.text_count += 1
        };
    }
}
