use std::{collections::HashMap, hash::Hash};
use serde::{Serialize, Deserialize};
use crate::{entities::EntityManager, g_properties::{Allegiances, GPAction, GPActionQueue, GPAllegiance, GPBuffBar, GPDebuff, GPDebuffBar, GPId, GPMortality, GPStats, GPTarget}, *};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameEntityManager {
    pub gids: HashMap<u32, GPId>,
    pub mortalities: HashMap<u32, GPMortality>,
    pub allegiances: HashMap<u32, GPAllegiance>,
    pub stats: HashMap<u32, GPStats>,
    pub targets: HashMap<u32, GPTarget>,
    pub buffs: HashMap<u32, g_properties::GPBuff>,
    pub buffbars: HashMap<u32, GPBuffBar>,
    pub debuffs: HashMap<u32, GPDebuff>,
    pub debuffbars: HashMap<u32, GPDebuffBar>,
    pub actions: HashMap<u32, GPAction>,
    pub actionqueue: HashMap<u32, GPActionQueue>,

    pub player_id: Option<u32>,
    game_entity_id_counter: u32,
    game_property_id_counter: u32,
}

impl GameEntityManager {
    pub fn new() -> Self {
        GameEntityManager {
            gids: HashMap::new(),
            mortalities: HashMap::new(),
            allegiances: HashMap::new(),
            stats: HashMap::new(),
            targets: HashMap::new(),
            buffs: HashMap::new(),
            buffbars: HashMap::new(),
            debuffs: HashMap::new(),
            debuffbars: HashMap::new(),
            actions: HashMap::new(),
            actionqueue: HashMap::new(),

            player_id: None,
            game_entity_id_counter: 0,
            game_property_id_counter: 0,
        }
    }

    pub fn get_entity_id_from_name(&mut self, tag: String) -> u32 {
        if let Some((&id, _)) = self.gids.iter().find(|(_, gid)| gid.tag == tag) {
            return id;
        }

        self.add_entity(Some(tag))
    }

    pub fn get_enemy(&mut self) -> Option<u32> {
        for (id, allegiance) in &self.allegiances {
            if allegiance.allegiance == Allegiances::Enemy  {
                return Some(*id);
            }
        }
        None
    }

    pub fn get_all_enemies(&self) -> Vec<u32> {
        self.allegiances
            .iter()
            .filter_map(|(id, allegiance)| {
                if allegiance.allegiance == Allegiances::Enemy {
                    Some(*id)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn add_entity(&mut self, tag: Option<String>) -> u32 {
        let id = self.next_eid();
        let final_tag = tag.unwrap_or_else(|| format!("entity_{}", id));
        self.gids.insert(id, GPId { id, tag: final_tag });
        id
    }

    pub fn next_eid(&mut self) -> u32 { // entity id
        let id = self.game_entity_id_counter;
        self.game_entity_id_counter += 1;
        id
    }

    pub fn next_pid(&mut self) -> u32 { // property id
        let id = self.game_property_id_counter;
        self.game_property_id_counter += 1;
        id
    }
}