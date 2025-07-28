use std::time::Instant;
use sfml::graphics::{Color, Font, RenderTarget, Text, Transformable};
use crate::{entities, properties::*};
use crate::game::*;
use crate::user_input::InputSlot;

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