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

const CHUNK_SIZE: usize = 1024; // u32s per chunk
const NUM_CHUNKS: usize = 256;
const STATE_SIZE: usize = CHUNK_SIZE * NUM_CHUNKS; // 262144

const CHUNK_FLAGS: usize = 0;      // 0..1023
const CHUNK_MOUSE: usize = 1;      // 1024..2047
const CHUNK_BUTTON_FLAGS: usize = 2; // 2048..3071

const LMB_NOW: usize = CHUNK_SIZE * CHUNK_FLAGS + 0;
const LMB_PREV: usize = CHUNK_SIZE * CHUNK_FLAGS + 1;

const MOUSE_X: usize = CHUNK_SIZE * CHUNK_MOUSE + 0;
const MOUSE_Y: usize = CHUNK_SIZE * CHUNK_MOUSE + 1;

const BUTTON_FLAGS_BASE: usize = CHUNK_SIZE * CHUNK_BUTTON_FLAGS;



fn main() {
    let mut g = game::Game::new();
    g.init();
    g.run();
    //let mut em = EntityManager::new();
    //em.add_entity("player".to_string(), entities::EntityType::Player);
    //let mut p = em.get_entity_from_string("player").unwrap();

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
