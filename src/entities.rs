use std::{collections::HashMap, hash::Hash};
use serde::{Serialize, Deserialize};
use sfml::graphics::Color;
use crate::{game::{MAIN_OUTLINE_CLR, MAIN_TEXT_CLR}, properties::*, helpers::*};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EntityManager {
    pub ids: HashMap<u32, PId>,                   // 1..1
    pub rectangles: HashMap<u32, Vec<PRect>>,     // 0..n
    pub texts: HashMap<u32, Vec<PText>>,          // 0..n
    pub healthbars: HashMap<u32, PHealthbar>,     // 0..1
    pub castbars: HashMap<u32, PCastbar>,         // 0..1
    pub state_vecs: HashMap<u32, PState>,         // 0..1
    pub tooltip_data: HashMap<u32, PTooltipData>, // 0..1
    pub clickables: HashMap<u32, PClickable>,

    entity_id_counter: u32,
    property_id_counter: u32,
}

impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            ids: HashMap::new(),
            rectangles: HashMap::new(),
            texts: HashMap::new(),
            healthbars: HashMap::new(),
            castbars: HashMap::new(),
            state_vecs: HashMap::new(),
            tooltip_data: HashMap::new(),
            clickables: HashMap::new(),

            entity_id_counter: 0,
            property_id_counter: 0,
        }
    }

    pub fn add_entity(&mut self, tag: Option<String>) -> u32 {
        let id = self.next_eid();
        let final_tag = tag.unwrap_or_else(|| format!("entity_{}", id));
        self.ids.insert(id, PId { id, tag: final_tag });
        id
    }

    fn next_eid(&mut self) -> u32 { // entity id
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;
        id
    }

    fn next_pid(&mut self) -> u32 { // property id
        let id = self.property_id_counter;
        self.property_id_counter += 1;
        id
    }

    pub fn add_property_to_entity(&mut self, prop: PropertiesEnum, id: u32) {
        match prop {
            PropertiesEnum::Rect => self.add_rect(id),
            PropertiesEnum::Text => self.add_text(id),
            PropertiesEnum::Healthbar => self.add_healthbar(id),
            PropertiesEnum::Castbar => self.add_castbar(id),
            PropertiesEnum::State => self.add_state(id),
            PropertiesEnum::TooltipData => self.add_tooltip(id),
            PropertiesEnum::Clickable => self.add_clickable(id),
        }
    }

    fn add_rect(&mut self, id: u32) {
        let pid = self.next_pid();
        self.rectangles.entry(id).or_default().push(PRect {
            id: pid,
            x: 10,
            y: 10,
            width: 10,
            height: 10,
            colors: ColorPair { fill: (19, 81, 150), outline: (24, 26, 28) },
            hovered_color: None,
            pressed_color: None,
            hovered: None,
            pressed: None,
            draw: false,
            strata: 0,
        });
    }

    fn add_text(&mut self, id: u32) {
        let pid = self.next_pid();
        self.texts.entry(id).or_default().push(PText {
            id: pid,
            text: "Run Code".to_string(),
            scale: 1,
            x: 50,
            y: 50,
            colors: ColorPair { fill: (255, 255, 255), outline: (0, 0, 0) },
            draw: false,
            strata: 0,
        });
    }

    fn add_healthbar(&mut self, id: u32) {
        let pid = self.next_pid();
        self.healthbars.insert(id, PHealthbar {
            id: pid,
            x: 10,
            y: 10,
            width: 10,
            height: 10,
            draw: true,
            strata: 20,
            base_colors: ColorPair { fill: (254, 0, 0), outline: (0, 0, 0) },
            inner_colors: ColorPair { fill: (0, 254, 0), outline: (0, 0, 0) },
            gem_entity_id: None,
        });
    }

    fn add_castbar(&mut self, id: u32) {
        let pid = self.next_pid();
        self.castbars.insert(id, PCastbar {
            id: pid,
            x: 10,
            y: 10,
            width: 10,
            height: 10,
            cast_progress: 0.0,
            icon_name: "miasma".to_string(),
            draw: true,
            strata: 20,
            base_colors: ColorPair { fill: (254, 0, 0), outline: (0, 0, 0) },
            inner_colors: ColorPair { fill: (0, 254, 0), outline: (0, 0, 0) },
        });
    }

    fn add_state(&mut self, id: u32) {
        let pid = self.next_pid();
        self.state_vecs.insert(id, PState {
            id: pid,
            state_vec: vec![0; 500],
        });
    }

    fn add_tooltip(&mut self, id: u32) {
        let pid = self.next_pid();
        self.tooltip_data.insert(id, PTooltipData {
            id: pid,
            header: "asd".to_string(),
            body: "asd".to_string(),
            x: 10,
            y: 10,
            width: 10,
            height: 10,
            icon: None,
        });
    }

    fn add_clickable(&mut self, id: u32) {
        let pid = self.next_pid();
        self.clickables.insert(id, PClickable {
            id: (pid), clickable: (true), rect_reference_id: (None), action: (ClickAction::RunButton) });
    }

    pub fn get_player_id(&self) -> Option<u32> {
        self.ids.iter()
            .find_map(|(&id, pid)| if pid.tag == "player" { Some(id) } else { None })
    }
    
    pub fn get_prects_mut(&mut self, id: u32) -> Option<&mut Vec<PRect>> {
        self.rectangles.get_mut(&id)
    }

    pub fn get_ptexts_mut(&mut self, id: u32) -> Option<&mut Vec<PText>> {
        self.texts.get_mut(&id)
    }

    pub fn get_phealthbar_mut(&mut self, id: u32) -> Option<&mut PHealthbar> {
        self.healthbars.get_mut(&id)
    }

    pub fn get_pcastbar_mut(&mut self, id: u32) -> Option<&mut PCastbar> {
        self.castbars.get_mut(&id)
    }

    pub fn get_pstate_mut(&mut self, id: u32) -> Option<&mut PState> {
        self.state_vecs.get_mut(&id)
    }

    pub fn get_tooltip_data_mut(&mut self, id: u32) -> Option<&mut PTooltipData> {
        self.tooltip_data.get_mut(&id)
    }

    pub fn get_pclickable_mut(&mut self, id: u32) -> Option<&mut PClickable> {
        self.clickables.get_mut(&id)
    }

    pub fn get_pid_mut(&mut self, id: u32) -> Option<&mut PId> {
        self.ids.get_mut(&id)
    }

    pub fn get_prects_non_mut(&self, id: u32) -> Option<&Vec<PRect>> {
        self.rectangles.get(&id)
    }

    pub fn get_ptexts_non_mut(&self, id: u32) -> Option<&Vec<PText>> {
        self.texts.get(&id)
    }

    pub fn get_phealthbar_non_mut(&self, id: u32) -> Option<&PHealthbar> {
        self.healthbars.get(&id)
    }

    pub fn get_pcastbar_non_mut(&self, id: u32) -> Option<&PCastbar> {
        self.castbars.get(&id)
    }

    pub fn get_pstate_non_mut(&self, id: u32) -> Option<&PState> {
        self.state_vecs.get(&id)
    }

    pub fn get_tooltip_data_non_mut(&self, id: u32) -> Option<&PTooltipData> {
        self.tooltip_data.get(&id)
    }

    pub fn get_pclickable_non_mut(&self, id: u32) -> Option<&PClickable> {
        self.clickables.get(&id)
    }

    pub fn get_pid_non_mut(&self, id: u32) -> Option<&PId> {
        self.ids.get(&id)
    }

    pub fn get_button_rect_non_mut(&self, entity_id: u32) -> Option<&PRect> {
        let rect_id = self.clickables.get(&entity_id)?.rect_reference_id?;
        let rects = self.rectangles.get(&entity_id)?;
        rects.iter().find(|r| r.id == rect_id)
    }

    pub fn get_prects_mut_by_tag(&mut self, tag: &str) -> Option<&mut Vec<PRect>> {
        let entity_id = self.ids.iter()
            .find_map(|(&eid, pid)| if pid.tag == tag { Some(eid) } else { None })?;

        self.rectangles.get_mut(&entity_id)
    }

    pub fn get_prects_by_name(&mut self, name: &str) -> Option<&mut Vec<PRect>> {
        let entity_id = self.ids.iter()
            .find_map(|(&id, pid)| if pid.tag == name { Some(id) } else { None })?;

        self.rectangles.get_mut(&entity_id)
    }

    pub fn create_button(&mut self, nm: Option<String>) -> u32 {
        let eid = self.add_entity(nm);
        self.add_rect(eid);
        self.add_text(eid);
        self.add_tooltip(eid);
        self.add_clickable(eid);
        return eid;
    }

    pub fn get_all_buttons(&self) -> Vec<u32> {
        self.ids
            .keys()
            .copied()
            .filter(|&id| {
                self.rectangles.contains_key(&id)
                    && self.texts.contains_key(&id)
                    && self.tooltip_data.contains_key(&id)
                    && self.clickables.contains_key(&id)
            })
            .collect()
    }

    pub fn get_button_rect_mut(&mut self, entity_id: u32) -> Option<&mut PRect> {
        let rect_id = self.clickables.get(&entity_id)?.rect_reference_id?;
        let rects = self.rectangles.get_mut(&entity_id)?;
        rects.iter_mut().find(|r| r.id == rect_id)
    }

}