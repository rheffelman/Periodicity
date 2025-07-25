use sfml::cpp::FBox;
use sfml::graphics::{Color, Font, Text, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable};
use sfml::window::{Event, Style};
use sfml::window::mouse::Button;
use std::fs;
use std::mem;
use serde::{Serialize, Deserialize};
use crate::entities::EntityManager;

mod entities;
mod properties;
mod systems;
mod game;

#[link(name = "Advapi32")]
unsafe extern "system" {}

fn main() {
    let mut g = game::Game::new(3840, 2160);
    g.init();
    g.run();
}

// STATE DEFINITIONS

// 1. state[0] == 1 means LMB (Left Mouse Button) is currently pressed (held down); 0 if not pressed
// 2. state[1] == 1 means LMB was pressed in the previous frame; 0 if not
//
// (NOTE: state[1] is NOT RMB anymore, it's LMB previous state, used for edge detection.)
//
// 3. state[len(state) / 4 + region_id] == 1 means the button with region_id was just pressed in this frame (set by handle_user_input, cleared after handle_button_presses runs)
//
// 4. state[len(state) / 2] is the current X position of the mouse, as last updated by any mouse button press
// 5. state[len(state) / 2 + 1] is the current Y position of the mouse, as last updated by any mouse button press
