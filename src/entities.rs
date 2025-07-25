use std::collections::HashMap;
use std::hash::Hash;
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
    pub tag: String,
    pub id: u32,
    etype: EntityType
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EntityManager {
    pub entities: Vec<Entity>,
    pub rectangles: HashMap<u32, PRect>,
    pub texts: HashMap<u32, Vec<PText>>,
    pub sprites: HashMap<u32, PSprite>,
    pub stats: HashMap<u32, PStats>,
    pub healthbars: HashMap<u32, PHealthbar>,
    pub castbars: HashMap<u32, PCastbar>,
    pub state_vecs: HashMap<u32, PState>,
    pub tooltip_data: Vec<PTooltipData>,
    entity_id_counter: u32,
}

impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            entities: Vec::new(),
            rectangles: HashMap::new(),
            texts: HashMap::new(),
            sprites: HashMap::new(),
            stats: HashMap::new(),
            healthbars: HashMap::new(),
            castbars: HashMap::new(),
            state_vecs: HashMap::new(),
            tooltip_data: Vec::new(),
            entity_id_counter: 0
        }
    }

    pub fn add_entity(&mut self, name: String, entitytype: EntityType) -> u32 {
        let id = self.entity_id_counter;
        self.entities.push(Entity { tag: name, id, etype: entitytype });
        self.entity_id_counter += 1;
        return id
    }

    pub fn add_property_to_entity(&mut self, c: PropertiesEnum, id: u32) {
        match c {
            PropertiesEnum::rect => {
                self.rectangles.insert(id, PRect {
                    x: 10,
                    y: 10,
                    width: 10,
                    height: 10,
                    colors: ColorPair { fill: (19, 81, 150), outline: (24, 26, 28) },
                    draw: false,
                    strata: 0,
                });
            }
            PropertiesEnum::text => {
                self.texts.entry(id).or_default().push(PText {
                    text: "Run Code".to_string(),
                    scale: 1,
                    x: 50,
                    y: 50,
                    colors: ColorPair {
                        fill: (255, 255, 255),
                        outline: (0, 0, 0),
                    },
                    draw: false,
                    strata: 0,
                });
            }

            PropertiesEnum::sprite => {
                self.sprites.insert(id, PSprite {
                    x: 10,
                    y: 10,
                    scale: 1,
                    sprite_name: "goosey".to_string(),
                    draw: false,
                    strata: 0,
                });
            }
            PropertiesEnum::stat => {
                self.stats.insert(id, PStats { health_max: (100), health_curr: (100), chaos: (1), solidity: (1), vitality: (1), haste: (1), will: (1), volatility: (1)});
            }
            PropertiesEnum::healthbar => {
                self.healthbars.insert(id, PHealthbar { x: (10), y: (10), width: (10), height: (10), base_colors: (ColorPair { fill: (254, 0, 0), outline: (0, 0, 0) }), inner_colors: (ColorPair { fill: (0, 254, 0), outline: (0, 0, 0) }), draw: (true), strata: (20) });
            }
            PropertiesEnum::castbar => {
                self.castbars.insert(id, PCastbar {
                    x: 10,
                    y: 10,
                    width: 10,
                    height: 10,
                    cast_progress: 0.0,
                    base_colors: ColorPair { fill: (254, 0, 0), outline: (0, 0, 0) },
                    inner_colors: ColorPair { fill: (0, 254, 0), outline: (0, 0, 0) },
                    icon_name: "miasma".to_string(),
                    draw: true,
                    strata: 20,
                });
            }
            PropertiesEnum::state => {self.state_vecs.insert(id, PState { state_vec: (vec![0; 500]) });}

            _ => {}
        }
    }

    pub fn get_prect(&self, e: u32) -> Option<&PRect> {
        return self.rectangles.get(&e)
    }

    pub fn get_ptexts(&self, e: u32) -> Option<&Vec<PText>> {
        self.texts.get(&e)
    }

    pub fn get_psprite(&self, e: u32) -> Option<&PSprite> {
        return self.sprites.get(&e);
    }

    pub fn get_entity_mut(&mut self, id: u32) -> Option<&mut Entity> {
        return self.entities.iter_mut().find(|e| e.id == id)
    }
    
}