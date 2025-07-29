use std::time::Instant;
use sfml::graphics::{Color, Font, RenderTarget, Text, Transformable};
use crate::{entities, properties::*};
use crate::game::*;
use crate::user_input::InputSlot;
use rand::Rng;

pub fn random_point_in_rect(width: u32, height: u32) -> (u32, u32) {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..width);
    let y = rng.gen_range(0..height);
    (x, y)
}

pub fn get_stat(n: u32) -> Option<String> {
    if n == 2 {
        return Some("chaos".to_string());
    }
    else if n == 3 {
        return Some("solidity".to_string());
    }
    else if n == 4 {
        return Some("vitality".to_string());
    }
    else if n == 5 {
        return Some("haste".to_string());
    }
    else {
        return None;
    }
}
pub fn get_stat_color(n: u32) -> Option<(u8, u8, u8)> {
    if n == 2 {
        return Some((EPIC.r, EPIC.g, EPIC.b));
    }
    else if n == 3 {
        return Some((122, 122, 115));
    }
    else if n == 4 {
        return Some((224, 65, 52));
    }
    else if n == 5 {
        return Some((240, 190, 88));
    }
    else {
        return None;
    }
}

pub fn wrap_text(text: &str, font: &Font, char_size: u32, max_width: f32) -> String {
    let mut wrapped = String::new();
    let mut line = String::new();
    let mut temp_text = Text::new("", font, char_size);

    for word in text.split_whitespace() {
        let try_line = if line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", line, word)
        };
        temp_text.set_string(&try_line);
        let width = temp_text.local_bounds().width;

        if width > max_width {
            if !line.is_empty() {
                wrapped.push_str(&line);
                wrapped.push('\n');
            }
            line = word.to_string();
        } else {
            line = try_line;
        }
    }

    if !line.is_empty() {
        wrapped.push_str(&line);
    }
    wrapped
}

pub fn to_rgb(c: Color) -> (u8, u8, u8) {
    (c.r, c.g, c.b)
}