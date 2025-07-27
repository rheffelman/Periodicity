use std::collections::HashMap;
use std::mem;
use std::time::Instant;

use sfml::cpp::FBox;
use sfml::graphics::{Color, Font, Texture, RenderWindow};
use sfml::graphics::{Text, RectangleShape, Sprite, RenderTarget, Transformable, Shape};
use sfml::window::{Style};

use crate::{entities, g_entities};
use crate::properties::*;
use crate::user_input::*;
use crate::construct_window::*;
use crate::animation::Animator;

pub static BASE: Color    = Color::rgba(36, 41, 46, 255);
pub static LIGHTER: Color = Color::rgba(43, 49, 55, 255);
pub static ENCAPSULATION_REGIONS: Color  = Color::rgba(29, 33, 37, 255);
pub static BUTTON: Color  = Color::rgba(4,66,137,255);
pub static BUTTON_PRESSED: Color = Color::rgba(127,184,251,255);
pub static BUTTON_HOVERED: Color = Color::rgba(0,92,197,255);
pub static ALT_BUTTON: Color = Color::rgba(243,88,2,255);
pub static ALT_BUTTON_HOVERED: Color = Color::rgba(239, 157, 112, 255);
pub static MAIN_OUTLINE_CLR: Color  = Color::rgba(24, 26, 28, 255);
pub static OFF_OUTLINE_CLR: Color = Color::rgba(107,117,127,255);
pub static LEGENDARY: Color = Color::rgba(250,214,104,255);
pub static EPIC: Color = Color::rgba(112,36,163,255);
pub static MAIN_TEXT_CLR: Color = Color::rgba(246,248,250,255); 
pub static OFF_TEXT_CLR: Color = Color::rgba(101,126,150,255);
pub static MIASMA_COLOR: Color = Color::rgba(125,185,112,255);
pub static INFERNUM_COLOR: Color = Color::rgba(233,103,6,255);

pub static WINDOW_WIDTH: u32 = 1920;
pub static WINDOW_HEIGHT: u32  = 1080;

pub fn get_scale() -> u32 {
    let scale_w = WINDOW_WIDTH as f32 / 1920.0;
    let scale_h = WINDOW_HEIGHT as f32 / 1080.0;
    scale_w.min(scale_h).floor().max(1.0) as u32
}

pub fn scaled(scale: u32, x: u32) -> u32 {
    x * scale
}

#[derive(Debug)]
pub struct Game {
    pub window: FBox<RenderWindow>,
    pub window_width: u32,
    pub window_height: u32,
    state: Vec<u32>,
    pub user_input_cache: Vec<u32>,
    pub input_index: usize,
    pub em: entities::EntityManager,
    pub gem: g_entities::GameEntityManager,
    pub em_gem_link: HashMap<u32, u32>,
    pub fnt: FBox<Font>,
    pub textures: HashMap<String, FBox<Texture>>,
    pub animator: Animator,

    pub time_elapsed: f32,    // total time in seconds (float)
    pub delta_time: f32,      // delta time in seconds (float)
    pub time_elapsed_ms: u64, // total time in milliseconds (int)
    last_frame_time: Instant,
}

impl Game {
    pub fn new(w_width: u32, w_height: u32) -> Game {
        let mut window = RenderWindow::new(
            (w_width, w_height),
            "Periodicity",
            Style::CLOSE,
            &Default::default(),
        )
        .expect("Failed to create SFML RenderWindow");
        window.set_vertical_sync_enabled(true);

        let font = Font::from_file("./src/assets/lilex.ttf")
            .expect("Failed to load font");
        
        let one_mb_bytes: usize = 1024 * 1024;
        let u32_size: usize = mem::size_of::<u32>();
        let num_elements: usize = one_mb_bytes / u32_size;

        let state_vec: Vec<u32> = vec![0; num_elements];
        let user_input_vec: Vec<u32> = vec![0; num_elements];

        Game {
            window,
            window_width: w_width,
            window_height: w_height,
            state: state_vec,
            user_input_cache: user_input_vec,
            input_index: 0,
            em: entities::EntityManager::new(),
            gem: g_entities::GameEntityManager::new(),
            em_gem_link: HashMap::new(),
            fnt: font,
            textures: HashMap::new(),
            animator: Animator::new(),

            time_elapsed: 0.0,
            delta_time: 0.0,
            time_elapsed_ms: 0,
            last_frame_time: Instant::now(),
        }
    }

    pub fn run(&mut self) {
        while self.window.is_open() {
            self.user_input_main_entry();
            self.update_game_main_entry();
            let now = Instant::now();
            let frame_duration = now - self.last_frame_time;

            self.delta_time = frame_duration.as_secs_f32();
            self.time_elapsed += self.delta_time;
            self.time_elapsed_ms += frame_duration.as_millis() as u64;
            self.last_frame_time = now;

            let progress = (self.time_elapsed % 5.0) / 5.0;

            for (_, castbar) in self.em.castbars.iter_mut() {
                castbar.cast_progress = progress;
            }
            self.animator.update(self.delta_time);
            self.render_main_entry();
        }
    }
}
