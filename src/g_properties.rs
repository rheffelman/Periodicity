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
pub enum StatsEnum {
    Chaos,
    Solidity,
    Vitality,
    Haste,
    Will,
    Volatility,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, )]
pub struct GPDebuffBar {
    pub id: u32,
    pub debuffs: Vec<GPDebuff>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, )]
pub struct GPDebuff {
    pub id: u32,
    pub name: String,
    pub total_duration: u32,
    pub time_left: u32,
    pub stacks: u32,
    pub pending_damage: f32,
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
    pub upfront_dam: u32,
    pub coefficient: u32,
    pub dps: u32,
    pub duration: u32,

}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GPLevel {
    pub id: u32,
    pub curr_level: u32,
    pub curr_xp: u32,
    pub next_level_xp: u32,
}

pub fn get_spelldata_from_string(spell: &String) -> Option<SpellData> {
    if spell == "miasma" || spell == "Miasma" {
        return get_spell_data(Spells::Miasma);
    }
    else if spell == "infernum" || spell == "Infernum" {
        return get_spell_data(Spells::Infernum);
    }
    return None;
}

pub fn get_spell_data(sp: Spells) -> Option<SpellData> {
    if sp == Spells::Miasma {
        Some(SpellData {
            icon: "miasma".to_string(),
            colors: ColorPair::from_colors(MIASMA_COLOR, Some(MAIN_OUTLINE_CLR)),
            upfront_dam: 0,
            coefficient: 3,
            dps: 5,
            duration: 5,
        })
    }
    else if sp == Spells::Infernum {
        Some(SpellData { 
            icon: ("infernum".to_string()), 
            colors: (ColorPair::from_colors(INFERNUM_COLOR, Some(MAIN_OUTLINE_CLR))),
            upfront_dam: 5,
            coefficient: 2,
            dps: 1,
            duration: 10 })
    }
    else {
        None
    }
}
