use crate::{entities, game::*, properties::{ColorPair, PText}};
use sfml::graphics::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Damage {
    pub amt: u32,
    pub target: u32,
    pub damager: u32,
    pub damage_type: String,
}

pub fn get_color_from_type (dtype: String) -> Option<Color> {
    if dtype == "miasma".to_string() {
        return Some(Color::rgba(1,255,150,255));
    }
    else if dtype == "infernum".to_string() {
        return Some(Color::rgba(255,125,10,255));
    }
    else if dtype == "umbra_mortis".to_string() {
        return Some(Color::rgba(148,114,201,255));
    }
    return None;
}
pub struct FloatingText {
    pub value: String,
    pub position: (f32, f32),
    pub velocity: (f32, f32),
    pub scale: u32,
    pub color: Color,
    pub outline: Color,
    pub lifetime: f32,
    //pub damage_type: Option<String>,
}

impl Game<'_> {
    // TODO add thing that sends icons in debuffbar to render
    pub fn s_debuffs(&mut self) {
        self.state_checker();
        
        // handle debuff damage
        let enemies = self.gem.get_all_enemies();
        for enemy in enemies {
            let mut dbb = self.gem.debuffbars.get_mut(&enemy);
            if dbb.is_none() { // skip if entity doesn't have a debuffbar
                continue;
            }
            if dbb.as_ref().unwrap().debuffs.is_empty() { // skip if debuffbar vec is empty
                continue;
            }

            // remove expired debuffs
            dbb.as_mut().unwrap().debuffs.retain(|debuff| debuff.time_left > 0);

            for debuff in dbb.as_mut().unwrap().debuffs.iter_mut() {
                if let Some(spell) = crate::g_properties::get_spelldata_from_string(&debuff.name.clone()) {
                    if let Some(e_stats) = self.gem.stats.get_mut(&enemy) { // get enemy stats to decrement health
                        
                        let dt_sec = self.delta_time_ms as f32 / 1000.0;
                        debuff.pending_damage += spell.dps as f32 * dt_sec * spell.coefficient as f32;

                        let whole_damage = debuff.pending_damage.floor() as u32;
                        debuff.pending_damage -= whole_damage as f32;
                        let dtype: String = debuff.name.clone();
                        self.damage_queue.push(Damage { 
                            amt: (whole_damage), 
                            target: (enemy), 
                            damager: (self.gem.player_id.unwrap()), 
                            damage_type: (dtype) });
                    }
                }

                debuff.time_left = debuff.time_left.saturating_sub(self.delta_time_ms);
            }
        }
    }

    pub fn s_damage(&mut self) {
        for damage_event in self.damage_queue.clone() {
            if let Some(target_stats) = self.gem.stats.get_mut(&damage_event.target) {
                if target_stats.health_curr > damage_event.amt {
                    println!("{}", damage_event.amt);
                    target_stats.health_curr -= damage_event.amt;
                }
                else {
                    target_stats.health_curr = 0;
                }
            }
                self.floating_combat_text(damage_event.amt, damage_event.damage_type);
        }
        
        self.damage_queue.clear();
    }

    pub fn s_mortality(&mut self) {
        // scan all entities with stats, if they have 0 health, they are dead.
        for (entity_id, stats) in self.gem.stats.iter() {
            if stats.health_curr == 0 {
                if let Some(mortality) = self.gem.mortalities.get_mut(entity_id) {
                    mortality.is_alive = false;
                }
            }
        }
    }

    fn floating_combat_text(&mut self, amt: u32, dmgtype: String) {
        if amt == 0 {
            return;
        }
        let scale_w = self.window_width as f32 / 1920.0;
        let scale_h = self.window_height as f32 / 1080.0;
        let scale = scale_w.min(scale_h).floor().max(1.0) as u32;
        let s = |x: u32| x * scale;
        let color = crate::systems::get_color_from_type(dmgtype);
        let actual_color: Color;
        if color.is_none() {
            actual_color = Color::WHITE;
        }
        else {
            actual_color = color.unwrap();
        }

        let x = crate::helpers::random_point_in_rect(s(400), s(200));
        self.floating_texts.push(FloatingText {
            value: amt.to_string(),
            position: ((x.0 + s(1400)) as f32, (x.1 + s(200)) as f32),
            velocity: (0.0, -30.0),
            scale: s(50),
            color: actual_color,
            outline: Color::BLACK,
            lifetime: 1.0,

        });
    }

    fn state_checker(&mut self) {
        if self.state[0] == 1 { // miasma
            self.miasma();
            self.state[0] = 0;
        }
        if self.state[1] == 1 {
            self.infernum();
            self.state[1] = 0;
        }
    }

    fn miasma(&mut self) {
        let enemy_id = self.gem.get_enemy();
        let next_id = self.gem.next_pid();
        self.gem.debuffbars.get_mut(&enemy_id.unwrap()).unwrap().debuffs.push(crate::g_properties::GPDebuff {
            id: (next_id),
            name: ("miasma".to_string()),
            total_duration: (4000),
            time_left: (4000),
            stacks: (1),
            pending_damage: 0.0 });
        
            self.add_icon_to_debuffbar("miasma".to_string());
    }

    fn infernum(&mut self) {
        let enemy_id = self.gem.get_enemy().unwrap();
        let next_id = self.gem.next_pid();

        // deal upfront damage
        self.damage_queue.push(Damage { 
            amt: (crate::g_properties::get_spell_data(crate::g_properties::Spells::Infernum).unwrap().upfront_dam), 
            target: (enemy_id), 
            damager: (self.gem.player_id.unwrap()), 
            damage_type: ("infernum".to_string()) });

        // add debuff
        let spell_duration = crate::g_properties::get_spell_data(crate::g_properties::Spells::Infernum).unwrap().duration;
        self.gem.debuffbars.get_mut(&enemy_id).unwrap().debuffs.push(crate::g_properties::GPDebuff {
            id: (next_id),
            name: ("infernum".to_string()),
            total_duration: (spell_duration * 1000),
            time_left: (spell_duration * 1000),
            stacks: (1),
            pending_damage: 0.0 });
        
            self.add_icon_to_debuffbar("infernum".to_string());
    }

    fn add_icon_to_debuffbar(&mut self, spellname: String) {
        let scale_w = WINDOW_WIDTH as f32 / 1920.0;
        let scale_h = WINDOW_HEIGHT as f32 / 1080.0;
        let scale = scale_w.min(scale_h).floor().max(1.0) as u32;
        let s = |x: u32| x * scale;

        let enemy_id = self.gem.get_enemy().unwrap();
        let num_curr_debuffs = self.gem.debuffbars.get(&enemy_id).unwrap().debuffs.len() - 1;
        let spell_duration = crate::g_properties::get_spelldata_from_string(&spellname).unwrap().duration;
        
        self.anims.add_animation_instance(crate::animation::AnimatedSprite {
            texture_id: spellname,
            frame_width: 64,
            frame_height: 64,
            total_frames: 1,
            current_frame: 0,
            frame_time: Some(spell_duration as f32),
            time_accumulator: 0.0,
            position: (s(1403)+ (num_curr_debuffs as u32 * s(64)) , s(647)),
            inanimate: true,
            strata: 30,
            desired_width: Some(s(64)),
            desired_height: Some(s(64)),
            play_once: true,
            finished: false,
            velocity: (0.0, 0.0),
            lifetime: Some(spell_duration as f32),
            associated_g_entity: None, });
    }
}