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
pub static BASE: Color    = Color::rgba(36, 41, 46, 255);
pub static LIGHTER: Color = Color::rgba(43, 49, 55, 255);
pub static ENCAPSULATION_REGIONS: Color  = Color::rgba(29, 33, 37, 255);
pub static BUTTON: Color  = Color::rgba(19, 81, 150, 255);
pub static BORDER: Color  = Color::rgba(24, 26, 28, 255);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EntityType {
    Button,
    Enemy,
    Player,
    Region,
    Sprite
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Entity {
    tag: String,
    pub id: u32,
    etype: EntityType
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EntityManager {
    pub entities: Vec<Entity>,
    pub positions: HashMap<u32, PPos>,
    pub rectangles: HashMap<u32, PRect>,
    pub texts: HashMap<u32, PText>,
    pub sprites: HashMap<u32, PSprite>,
    entity_id_counter: u32,
}

impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            entities: Vec::new(),
            positions: HashMap::new(),
            rectangles: HashMap::new(),
            texts: HashMap::new(),
            sprites: HashMap::new(),
            entity_id_counter: 0
        }
    }

    pub fn add_entity(&mut self, name: String, entitytype: EntityType) -> u32 {
        let id = self.entity_id_counter;
        self.entities.push(Entity { tag: name, id, etype: entitytype });
        self.entity_id_counter += 1;
        return id
    }

    pub fn add_component_to_entity(&mut self, c: PropertiesEnum, id: u32) {
        match c {
            PropertiesEnum::pos => { self.positions.insert(id, PPos { x: 0, y: 0 }); }
            PropertiesEnum::rect => { self.rectangles.insert(id, PRect { x: 10, y: 10, width: 10, height: 10, r: 19, g: 81, b: 150, draw: false, strata: 0 }); }
            PropertiesEnum::text => { self.texts.insert(id, PText { r: (255), g: (255), b: (255), text: ("Run Code".to_string()), scale: (1), x: (50), y: (50), draw: false, strata: 0 }); }
            PropertiesEnum::sprite => { self.sprites.insert(id, PSprite {x: 10, y: 10, scale: 1, sprite_name: "goosey".to_string(), draw: false, strata: 0}); }
            _ => {}
        }
    }

    pub fn get_prect(&self, e: u32) -> Option<&PRect> {
        return self.rectangles.get(&e)
    }

    pub fn get_ptext(&self, e: u32) -> Option<&PText> {
        return self.texts.get(&e)
    }

    pub fn get_psprite(&self, e: u32) -> Option<&PSprite> {
        return self.sprites.get(&e);
    }

    pub fn get_entity_mut(&mut self, id: u32) -> Option<&mut Entity> {
        return self.entities.iter_mut().find(|e| e.id == id)
    }
    
}