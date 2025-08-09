use std::collections::HashMap;
use std::mem;
use std::time::Instant;

use sfml::cpp::FBox;
use sfml::graphics::{Color, Font, Texture, RenderWindow};
use sfml::graphics::{Text, RectangleShape, Sprite, RenderTarget, Transformable, Shape, Shader};
use sfml::window::{Style};
use std::rc::Rc;
use std::cell::RefCell;

use crate::animation::Animation;
use crate::{entities, g_entities};
use crate::properties::*;
use crate::user_input::*;
use crate::construct_window::*;

pub static BASE: Color    = Color::rgba(43,49,55,255);
pub static ALT_BASE: Color = Color::rgba(36,41,46,255);
pub static LIGHTER: Color = Color::rgba(43, 49, 55, 255);
pub static ENCAPSULATION_REGIONS: Color  = Color::rgba(29, 33, 37, 255);
pub static BUTTON: Color  = Color::rgba(4,66,137,255);
pub static BUTTON_PRESSED: Color = Color::rgba(127,184,251,255);
pub static BUTTON_HOVERED: Color = Color::rgba(0,92,197,255);
pub static ALT_BUTTON: Color = Color::rgba(243,88,2,255);
pub static ALT_BUTTON_HOVERED: Color = Color::rgba(243, 103, 25, 255);
pub static ALT_BUTTON_PRESSED: Color = Color::rgba(239, 157, 112, 255);
pub static MAIN_OUTLINE_CLR: Color  = Color::rgba(24, 26, 28, 255);
pub static OFF_OUTLINE_CLR: Color = Color::rgba(107,117,127,255);
pub static LEGENDARY: Color = Color::rgba(250,214,104,255);
pub static EPIC: Color = Color::rgba(112,36,163,255);
pub static MAIN_TEXT_CLR: Color = Color::rgba(246,248,250,255); 
pub static OFF_TEXT_CLR: Color = Color::rgba(101,126,150,255);
pub static MIASMA_COLOR: Color = Color::rgba(125,185,112,255);
pub static INFERNUM_COLOR: Color = Color::rgba(233,103,6,255);
pub static XP_COLOR: Color = Color::rgba(98,67,211,255);

pub static WINDOW_WIDTH: u32 = 3840;
pub static WINDOW_HEIGHT: u32  = 2160;

pub fn get_scale() -> u32 {
    let scale_w = WINDOW_WIDTH as f32 / 1920.0;
    let scale_h = WINDOW_HEIGHT as f32 / 1080.0;
    scale_w.min(scale_h).floor().max(1.0) as u32
}

pub fn scaled(scale: u32, x: u32) -> u32 {
    x * scale
}

// #[derive(Debug)]
pub struct Game<'a> {
    pub window: FBox<RenderWindow>,
    pub window_width: u32,
    pub window_height: u32,
    pub state: Vec<u32>,
    pub user_input_cache: Vec<u32>,
    pub input_index: usize,
    pub em: entities::EntityManager,
    pub gem: g_entities::GameEntityManager,
    pub em_gem_link: HashMap<u32, u32>,
    pub fnt: FBox<Font>,
    pub gbfnt: FBox<Font>,
    pub textures: HashMap<String, FBox<Texture>>,
    pub anims: Animation,
    pub damage_queue: Vec<crate::systems::Damage>,
    pub desat_shader: FBox<Shader<'a>>,
    pub floating_texts: Vec<crate::systems::FloatingText>,

    pub time_elapsed: f32,    // total time in seconds (float)
    pub delta_time: f32,      // delta time in seconds (float)
    pub time_elapsed_ms: u32, // total time in milliseconds (int)
    pub delta_time_ms: u32,
    last_frame_time: Instant,
    pub miasma_has_spawned: bool,
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        let mut window = RenderWindow::new(
            (WINDOW_WIDTH, WINDOW_HEIGHT),
            "Periodicity",
            Style::CLOSE,
            &Default::default(),
        )
        .expect("Failed to create SFML RenderWindow");
        window.set_vertical_sync_enabled(true);

        let font = Font::from_file("./src/assets/lilex.ttf")
            .expect("Failed to load font");
        let gbfont = Font::from_file("./src/assets/gb.ttf")
            .expect("Failed to load font");
        
        let one_mb_bytes: usize = 1024 * 1024;
        let u32_size: usize = mem::size_of::<u32>();
        let num_elements: usize = one_mb_bytes / u32_size;

        let state_vec: Vec<u32> = vec![0; num_elements];
        let user_input_vec: Vec<u32> = vec![0; num_elements];

        let mut shader = Shader::from_file("./src/shaders/desaturate.frag", sfml::graphics::ShaderType::Fragment)
            .expect("Failed to load shader");
        shader.set_uniform_float("desaturation", 1.0).unwrap();
        
        Game {
            window,
            window_width: WINDOW_WIDTH,
            window_height: WINDOW_HEIGHT,
            state: state_vec,
            user_input_cache: user_input_vec,
            input_index: 0,
            em: entities::EntityManager::new(),
            gem: g_entities::GameEntityManager::new(),
            em_gem_link: HashMap::new(),
            fnt: font,
            gbfnt: gbfont,
            textures: HashMap::new(),
            anims: Animation::new(),
            damage_queue: Vec::new(),
            desat_shader: shader,
            floating_texts: Vec::new(),

            time_elapsed: 0.0,
            delta_time: 0.0,
            time_elapsed_ms: 0,
            delta_time_ms: 0,
            last_frame_time: Instant::now(),
            miasma_has_spawned: false,
        }
    }

    pub fn run(&mut self) {
        while self.window.is_open() {
            self.user_input_main_entry(); // branch to user_input.rs

            // timekeeping
            let now = Instant::now();
            let frame_duration = now - self.last_frame_time;
            self.delta_time = frame_duration.as_secs_f32();
            self.delta_time_ms = frame_duration.as_millis().min(u32::MAX as u128) as u32;
            if self.delta_time_ms == 0 {
                self.delta_time_ms = 1;
            }
            self.time_elapsed += self.delta_time;
            self.time_elapsed_ms += self.delta_time_ms;
            self.last_frame_time = now;

            // game systems
            
            self.s_mortality(); // branch to systems.rs
            self.s_debuffs(); // branch to systems.rs
            self.s_damage(); // branch to systems.rs
            self.update_game_main_entry(); // branch to update_game.rs
            self.anims.update(self.delta_time);

            // render last
            self.render_main_entry(); // branch to render_pipeline.rs
        }
    }

}


// STATE DEFINITIONS:

// state[0] == miasma
// state[1] == infernum