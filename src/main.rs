use sfml::cpp::FBox;
use sfml::graphics::{Color, Font, Text, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable};
use sfml::window::{Event, Style};
use sfml::window::mouse::Button;

use std::fs;
use std::mem;
use serde::{Serialize, Deserialize};

use crate::entities::EntityManager;

mod entities;
mod components;
mod systems;

#[link(name = "Advapi32")]

unsafe extern "system" {}

pub static BASE: Color    = Color::rgba(36, 41, 46, 255);
pub static LIGHTER: Color = Color::rgba(43, 49, 55, 255);
pub static ENCAPSULATION_REGIONS: Color  = Color::rgba(29, 33, 37, 255);
pub static BUTTON: Color  = Color::rgba(19, 81, 150, 255);
pub static BORDER: Color  = Color::rgba(24, 26, 28, 255);

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


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct REncapsulation {
    region_id: u32,
    name: String,
    width: u32,
    height: u32,
    pos_x: u32,
    pos_y: u32,
}

impl REncapsulation {
    pub fn new(region_id: u32, name: String, width: u32, height: u32, pos_x: u32, pos_y: u32) -> Self {
        REncapsulation { region_id, name, width, height, pos_x, pos_y }
    }
    pub fn get_region_id(&self) -> u32 {
        self.region_id
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_width(&self) -> u32 {
        self.width
    }
    pub fn get_height(&self) -> u32 {
        self.height
    }
    pub fn get_pos_x(&self) -> u32 {
        self.pos_x
    }
    pub fn get_pos_y(&self) -> u32 {
        self.pos_y
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct R_Button {
    region_id: u32,
    name: String,
    width: u32,
    height: u32,
    pos_x: u32,
    pos_y: u32,
    text: Option<String>,
}

impl R_Button {
    pub fn new(region_id: u32, name: String, width: u32, height: u32, pos_x: u32, pos_y: u32, text: Option<String>) -> Self {
        R_Button { region_id, name, width, height, pos_x, pos_y, text }
    }
    pub fn get_region_id(&self) -> u32 {
        self.region_id
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_width(&self) -> u32 {
        self.width
    }
    pub fn get_height(&self) -> u32 {
        self.height
    }
    pub fn get_pos_x(&self) -> u32 {
        self.pos_x
    }
    pub fn get_pos_y(&self) -> u32 {
        self.pos_y
    }
    pub fn get_text(&self) -> Option<&str> {
        self.text.as_deref()
    }
}
pub struct Game {
    window: FBox<RenderWindow>,
    r_encapsulations: Vec<REncapsulation>,
    r_buttons: Vec<R_Button>,
    id_last: u32,
    font: FBox<Font>,
    state: Vec<u32>, // see state definitions at the bottom of file to make sense of this.
}

impl Game {
    fn new() -> Game {
        let mut window = RenderWindow::new(
            (1920, 1080),
            "RPGame",
            Style::CLOSE,
            &Default::default(),
        )
        .expect("Failed to create SFML RenderWindow");
        window.set_vertical_sync_enabled(true);

        let font = Font::from_file("./src/assets/lilex.ttf")
            .expect("Failed to load font");

        let id_last_init: u32 = 0;

        let one_mb_bytes: usize = 1024 * 1024;
        let u32_size: usize = mem::size_of::<u32>();
        let num_elements: usize = one_mb_bytes / u32_size;

        let state_vec: Vec<u32> = vec![0; num_elements];

        Game { window, r_encapsulations: Vec::new(), r_buttons: Vec::new(), id_last: id_last_init, font, state: state_vec }
    }

    fn init(&mut self) {
        self.load_r_encapsulations("./src/gamedata/r_encapsulations.json");
        self.load_r_buttons("./src/gamedata/r_buttons.json");
    }

    fn load_r_encapsulations(&mut self, fp: &str) {
        let data = fs::read_to_string(fp).expect("Unable to read encapsulation regions file");
        self.r_encapsulations = serde_json::from_str(&data).expect("JSON was not well-formatted");
        self.id_last = self.r_encapsulations.last().map_or(0, |r| r.get_region_id());
    }
    
    fn load_r_buttons(&mut self, fp: &str) {
        let data = fs::read_to_string(fp).expect("Unable to read buttons file");
        self.r_buttons = serde_json::from_str(&data).expect("JSON was not well-formatted");
    }

    fn run(&mut self) {
        while self.window.is_open() {
            self.cache_user_input();
            self.handle_user_input();
            self.handle_button_presses();
            self.render();
        }
    }

    fn cache_user_input(&mut self) {
        // Store previous LMB state
        self.state[LMB_PREV] = self.state[LMB_NOW];

        while let Some(event) = self.window.poll_event() {
            match event {
                Event::Closed => self.window.close(),

                Event::MouseButtonPressed { button, x, y } => {
                    self.state[MOUSE_X] = x as u32;
                    self.state[MOUSE_Y] = y as u32;
                    if button == Button::Left {
                        self.state[LMB_NOW] = 1;
                    }
                }
                Event::MouseButtonReleased { button, .. } => {
                    if button == Button::Left {
                        self.state[LMB_NOW] = 0;
                    }
                }
                _ => {}
            }
        }
    }

    fn handle_user_input(&mut self) {
        let lmb_now = self.state[LMB_NOW];
        let lmb_prev = self.state[LMB_PREV];

        if lmb_prev == 0 && lmb_now == 1 {
            let mouse_x = self.state[MOUSE_X];
            let mouse_y = self.state[MOUSE_Y];

            let mut matched = None;
            for (i, button) in self.r_buttons.iter().enumerate() {
                if mouse_x >= button.get_pos_x() && mouse_x <= button.get_pos_x() + button.get_width() &&
                    mouse_y >= button.get_pos_y() && mouse_y <= button.get_pos_y() + button.get_height() {
                    if matched.is_some() {
                        // More than one button under cursor—abort
                        return;
                    }
                    matched = Some(i);
                }
            }

            if let Some(button_idx) = matched {
                let region_id = self.r_buttons[button_idx].get_region_id() as usize;
                self.state[BUTTON_FLAGS_BASE + region_id] = 1;
            }
        }
    }

    fn handle_button_presses(&mut self) {
        for button in &self.r_buttons {
            let flag_index = BUTTON_FLAGS_BASE + button.get_region_id() as usize;
            if self.state[flag_index] == 1 {
                println!(
                    "Button '{}' with ID {} was pressed!",
                    button.get_name(),
                    button.get_region_id()
                );
                self.state[flag_index] = 0;
            }
        }
    }

    fn render(&mut self) {
        self.window.clear(BASE);

        // render GUI
        // Render Encapsulation Regions
        for region in &self.r_encapsulations {
            let mut region_shape = RectangleShape::new();
            region_shape.set_size((region.get_width() as f32, region.get_height() as f32));
            region_shape.set_position((region.get_pos_x() as f32, region.get_pos_y() as f32));
            region_shape.set_fill_color(ENCAPSULATION_REGIONS);
            region_shape.set_outline_color(BORDER);
            region_shape.set_outline_thickness(2.0);
            self.window.draw(&region_shape);
        }

        // Render Button Regions
        for button in &self.r_buttons {
            let mut button_shape = RectangleShape::new();
            button_shape.set_size((button.get_width() as f32, button.get_height() as f32));
            button_shape.set_position((button.get_pos_x() as f32, button.get_pos_y() as f32));
            button_shape.set_fill_color(BUTTON);
            button_shape.set_outline_color(BORDER);
            button_shape.set_outline_thickness(2.0);
            self.window.draw(&button_shape);
            if let Some(text) = button.get_text() {
                let mut button_text = Text::new(text, &self.font, 20);
                button_text.set_position((button.get_pos_x() as f32 + 10.0, button.get_pos_y() as f32 + 10.0));
                button_text.set_fill_color(Color::WHITE);
                self.window.draw(&button_text);
            }
        }

        self.window.display();
    }
}

fn main() {
    // let mut g = Game::new();
    // g.init();
    // g.run();
    let mut em = EntityManager::new();
    em.add_entity("player".to_string(), entities::EntityType::Player);
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
