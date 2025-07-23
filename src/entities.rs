use std::collections::HashMap;
use sfml::audio::listener::position;
use sfml::cpp::FBox;
use sfml::graphics::{Color, Font, Text, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable};
use sfml::window::{Event, Style};
use sfml::window::mouse::Button;

use std::fs;
use std::mem;
use serde::{Serialize, Deserialize};
use crate::{components::*};

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

pub struct EntityManager<'a> {
    entities: Vec<Entity>,
    positions: HashMap<Entity, CPos>,
    rectangles: HashMap<Entity, CRect<'a>>,
    entity_id_counter: u32
}

impl EntityManager<'_> {
    pub fn new() -> Self {
        EntityManager {
            entities: Vec::new(),
            positions: HashMap::new(),
            rectangles: HashMap::new(),
            entity_id_counter: 0
        }
    }

    pub fn add_entity(&mut self, name: String, entitytype: EntityType) -> u32 {
        self.entities.push(Entity { tag: name, id: self.entity_id_counter, etype: entitytype});
        self.entity_id_counter += 1;
        self.entity_id_counter - 1
    }

    pub fn add_component_to_entity(&mut self, c: Components, e: Entity) -> bool {
        if c == (Components::CPos) {
            self.positions.insert(e, CPos {x: 0,y:  0});
            true;
        }
        else if c == Components::CRect {
            let mut temp_rect = RectangleShape::new();
            self.rectangles.insert(e, CRect{rect: temp_rect});
        }

        false
    }

    pub fn get_crect(&mut self, e: Entity) -> CRect<'static>{
        return self.rectangles.get(&e);
    }
}