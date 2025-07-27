use serde::{Serialize, Deserialize};

use crate::{game::{INFERNUM_COLOR, MAIN_OUTLINE_CLR, MIASMA_COLOR, OFF_OUTLINE_CLR}, properties::ColorPair};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GPEnum {
    GPId
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GPId { // each entity must have 1 and only 1 PId
    pub id: u32,
    pub tag: String, // will be specified for you if not specified.
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Allegiances {
    Player,
    Enemy,
    Neutral,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GPAllegiance {
    pub id: u32,
    pub allegiance: Allegiances
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GPMortality {
    pub id: u32,
    pub is_alive: bool
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GPStats {
    pub id: u32,
    pub health_max: u32,
    pub health_curr: u32,
    pub chaos: u32,
    pub solidity: u32,
    pub vitality: u32,
    pub haste: u32,
    pub will: u32,
    pub volatility: u32
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GPTarget {
    pub id: u32,
    pub target_entity: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GPBuffBar {
    pub id: u32,
    pub buffs: Vec<GPBuff>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GPBuff {
    pub id: u32,
    pub name: String,
    pub duration: u32,
    pub stacks: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GPDebuffBar {
    pub id: u32,
    pub debuffs: Vec<GPDebuff>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GPDebuff {
    pub id: u32,
    pub name: String,
    pub duration: u32,
    pub stacks: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Actions {
    CastingSpell,
    Traveling,
    Mining
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GPAction {
    pub id: u32,
    pub action: Actions,
    pub action_tag: String,
    pub time_action_takes: u32,
    pub time_remaining: u32,
    pub spell: Option<Spells>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GPActionQueue {
    pub id: u32,
    pub queue: Vec<GPAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Spells {
    Miasma,
    Infernum,
    UmbraMortis,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SpellData {
    pub icon: String,
    pub colors: ColorPair,
}

pub fn get_spell_data(sp: Spells) -> Option<SpellData> {
    if sp == Spells::Miasma {
        Some(SpellData {
            icon: "miasma".to_string(),
            colors: ColorPair::from_colors(MIASMA_COLOR, MAIN_OUTLINE_CLR)
        })
    }
    else if sp == Spells::Infernum {
        Some(SpellData { 
            icon: ("infernum".to_string()), 
            colors: (ColorPair::from_colors(INFERNUM_COLOR, MAIN_OUTLINE_CLR)) })
    }
    else {
        None
    }
}