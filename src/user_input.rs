use std::collections::HashMap;
use sfml::audio::listener::position;
use sfml::cpp::FBox;
use sfml::graphics::{Color, Font, Text, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable};
use sfml::graphics::{Texture, Sprite};
use sfml::window::{Event, Style};
use sfml::window::mouse::Button;
use std::time::Instant;
use std::fs;
use std::mem;
use serde::{Serialize, Deserialize};
use crate::{entities, properties::*};
use crate::{game::*};
use std::sync::LazyLock;
pub static BASE: Color    = Color::rgba(36, 41, 46, 255);
pub static LIGHTER: Color = Color::rgba(43, 49, 55, 255);
pub static ENCAPSULATION_REGIONS: Color  = Color::rgba(29, 33, 37, 255);
pub static BUTTON: Color  = Color::rgba(19, 81, 150, 255);
pub static BORDER: Color  = Color::rgba(24, 26, 28, 255);

//use sfml::window::{Event, Key, MouseButton};

impl Game {
    pub fn cache_user_input(&mut self) {
        while let Some(event) = self.window.poll_event() {
            let code: u32 = match event {
                Event::Closed => {
                    self.window.close();
                    0
                }
                Event::KeyPressed { code, .. } => 1_000 + code as u32,
                Event::KeyReleased { code, .. } => 2_000 + code as u32,
                Event::MouseButtonPressed { button, .. } => 3_000 + button as u32,
                Event::MouseButtonReleased { button, .. } => 4_000 + button as u32,
                Event::MouseMoved { x, y } => {
                    let packed = ((x as u32) & 0xFFFF) | (((y as u32) & 0xFFFF) << 16);
                    5_000_000 + packed
                }
                Event::MouseWheelScrolled { delta, .. } => 6_000 + delta as u32,
                Event::Resized { width, height } => {
                    let packed = ((width as u32) & 0xFFFF) | (((height as u32) & 0xFFFF) << 16);
                    7_000_000 + packed
                }
                _ => continue,
            };
            let cache_size = self.user_input_cache.len();
            self.user_input_cache[self.input_index % cache_size] = code;
            self.input_index += 1;
        }
    }
}
