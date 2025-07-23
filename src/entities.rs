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
    Player
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]

pub struct Entity {
    tag: String,
    id: u32,
    etype: EntityType
}

pub struct EntityManager {
    pub entities: Vec<Entity>, // own the values
    pub positions: HashMap<u32, PPos>, // use id as key
    pub rectangles: HashMap<u32, PRect>,
    entity_id_counter: u32,
}


impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            entities: Vec::new(),
            positions: HashMap::new(),
            rectangles: HashMap::new(),
            entity_id_counter: 0
        }
    }


    pub fn add_entity(&mut self, name: String, entitytype: EntityType) -> u32 {
        let id = self.entity_id_counter;
        self.entities.push(Entity { tag: name, id, etype: entitytype });
        self.entity_id_counter += 1;
        id // Return id to caller
    }


    pub fn add_component_to_entity(&mut self, c: PropertiesEnum, id: u32) {
        match c {
            PropertiesEnum::pos => { self.positions.insert(id, PPos { x: 0, y: 0 }); }
            PropertiesEnum::rect => { self.rectangles.insert(id, PRect { x: 10, y: 10, width: 10, height: 10, c: BUTTON, draw: false }); }
            _ => {}
        }
    }


    pub fn get_crect(&mut self, e: u32) -> Option<&PRect> {
        return self.rectangles.get(&e)
    }

    pub fn get_entity_mut(&mut self, id: u32) -> Option<&mut Entity> {
        self.entities.iter_mut().find(|e| e.id == id)
    }

}