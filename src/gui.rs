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

impl Game {
    pub fn render_tooltips(&mut self) {
        let known_tooltips = self.em.tooltip_data.clone();

        // Find latest mouse move from the cache
        let mouse_pos_opt = self.user_input_cache
            .iter()
            .rev()
            .find(|&&val| val >= 5_000_000 && val < 6_000_000)
            .map(|&val| {
                let packed = val - 5_000_000;
                let x = (packed & 0xFFFF) as i32;
                let y = ((packed >> 16) & 0xFFFF) as i32;
                (x, y)
            });

        if let Some((mouse_x, mouse_y)) = mouse_pos_opt {
            for tooltip in known_tooltips {
                let x = tooltip.x as i32;
                let y = tooltip.y as i32;
                let w = tooltip.width as i32;
                let h = tooltip.height as i32;

                if mouse_x >= x
                    && mouse_x <= x + w
                    && mouse_y >= y
                    && mouse_y <= y + h
                {
                    // Mouse is over this tooltip
                    println!("Hovered tooltip: {}", tooltip.header);

                    // You can draw it here, or call self.draw_tooltip(&tooltip)
                }
            }
        }
    }
}
