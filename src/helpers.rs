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

impl<'a> Game<'a> {
    pub fn get_player_stats(&mut self, n: u32) -> Option<u32> {
        let pid = self.gem.player_id.unwrap();
        let player_stats = self.gem.stats.get(&pid).unwrap();

        match n {
            2 => Some(player_stats.chaos),
            3 => Some(player_stats.solidity),
            4 => Some(player_stats.vitality),
            5 => Some(player_stats.haste),
            6 => Some(player_stats.will),
            _ => None,
        }
    }
}

pub fn get_stat(n: u32) -> Option<String> {
    match n {
        2 => Some("chaos".to_string()),
        3 => Some("solidity".to_string()),
        4 => Some("vitality".to_string()),
        5 => Some("haste".to_string()),
        6 => Some("will".to_string()),
        _ => None,
    }
}
pub fn get_stat_color(n: u32) -> Option<(u8, u8, u8)> {
    match n {
        2 => Some((EPIC.r, EPIC.g, EPIC.b)),
        3 => Some((122, 122, 115)),
        4 => Some((224, 65, 52)),
        5 => Some((240, 190, 88)),
        6 => Some((198,38,65)),
        _ => None,
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