use std::collections::HashMap;
use sfml::audio::listener::position;
use sfml::cpp::FBox;
use sfml::graphics::{Color, Font, Text, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable};
use sfml::window::{Event, Style};
use sfml::window::mouse::Button;

use std::fs;
use std::mem;
use serde::{Serialize, Deserialize};
use crate::{entities, properties::*};
use std::sync::LazyLock;

pub static BASE: Color    = Color::rgba(36, 41, 46, 255);
pub static LIGHTER: Color = Color::rgba(43, 49, 55, 255);
pub static ENCAPSULATION_REGIONS: Color  = Color::rgba(29, 33, 37, 255);
pub static BUTTON: Color  = Color::rgba(19, 81, 150, 255);
pub static BORDER: Color  = Color::rgba(24, 26, 28, 255);

#[derive(Debug,)]
pub struct Game {
    window: FBox<RenderWindow>,
    state: Vec<u32>, // see state definitions at the bottom of file to make sense of this.
    em: entities::EntityManager,
    fnt: FBox<Font>
}

impl Game {
    pub fn new() -> Game {
        let mut window = RenderWindow::new(
            (1920, 1080),
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

        Game { window, state: state_vec, em: entities::EntityManager::new(), fnt: font }
    }

    pub fn init(&mut self) {
        self.spawn_player();
        self.init_gui();
    }

    pub fn init_gui(&mut self) {
        let rbid = self.em.add_entity("run_button".to_string(), entities::EntityType::Button);
        self.em.add_component_to_entity(PropertiesEnum::rect, rbid);

        if let Some(rect) = self.em.rectangles.get_mut(&rbid) {
            rect.width = 300;
            rect.height = 100;
            rect.x = 10;
            rect.y = 10;
            rect.draw = true;
        }

        self.em.add_component_to_entity(PropertiesEnum::text, rbid);

    }

    pub fn run(&mut self) {
        while self.window.is_open() {
            while let Some(event) = self.window.poll_event() {
                match event {
                    Event::Closed => self.window.close(),
                    _ => {}
                }
            }

            self.render();
        }
    }

    fn render(&mut self) {
        self.window.clear(entities::BASE);

        // iterate through all entities
        for e in self.em.entities.clone() {
            // if an entity has a rectangle property, and is flagged to draw it, then draw it.
            if self.em.get_prect(e.id).is_some() && self.em.get_prect(e.id).unwrap().draw == true {
                let e_rect = self.em.get_prect(e.id).unwrap();
                let mut draw_rect = RectangleShape::new();
                draw_rect.set_size((e_rect.width as f32, e_rect.height as f32));
                draw_rect.set_position((e_rect.x as f32, e_rect.y as f32));
                draw_rect.set_fill_color(BUTTON);
                self.window.draw(&draw_rect);
            }

            // if an entity has text, draw it
            if self.em.get_ptext(e.id).is_some() {
                let e_text = self.em.get_ptext(e.id).unwrap();
                let mut draw_text = Text::new(&e_text.text, &self.fnt, 20);
                draw_text.set_scale(1.0);
                draw_text.set_position((e_text.x as f32, e_text.y as f32));
                draw_text.set_fill_color(Color::rgb(e_text.r, e_text.g, e_text.b));
                self.window.draw(&draw_text);
            }
        }

        self.window.display();
    }

    fn spawn_player(&mut self) {
        let pid = self.em.add_entity("player".to_string(), entities::EntityType::Player);
        self.em.add_component_to_entity(PropertiesEnum::rect, pid);

        if let Some(rect) = self.em.rectangles.get_mut(&pid) {
            rect.width = 800;
            rect.height = 800;
            rect.x = 500;
            rect.y = 500;
            rect.draw = true;
        }
    }
}