use std::{collections::HashMap, hash::Hash};
use serde::{Serialize, Deserialize};
use crate::game::*;
use crate::g_properties::*;

impl Game {
    pub fn update_game_main_entry(&mut self) {
        let dt_ms = (self.delta_time * 1000.0) as u32;
        let dt = self.delta_time;
        self.animator.update(dt);

// Get the player entity ID and action
        let player_em_id = self.gem.player_id.unwrap();
        let Some(&em_id) = self.em_gem_link.get(&player_em_id) else { return; };
        let Some(sprite) = self.em.sprites.get_mut(&em_id) else { return; };

        if let Some(queue) = self.gem.actionqueue.get(&player_em_id) {
            if let Some(action) = queue.queue.first() {
                if action.action == Actions::CastingSpell && action.time_remaining > 0 {
                    sprite.anim_name = Some("Miasma_anim".to_string());
                } else {
                    sprite.anim_name = None;
                }
            } else {
                sprite.anim_name = None;
            }
        }

        // === Update ActionQueue (time_remaining) ===
        for (&gem_id, queue) in self.gem.actionqueue.iter_mut() {
            let remove = {
                if let Some(action) = queue.queue.first_mut() {
                    if action.action == Actions::CastingSpell {
                        if action.time_remaining > dt_ms {
                            action.time_remaining -= dt_ms;
                            false
                        } else {
                            action.time_remaining = 0;
                            true
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            };

            if remove {
                queue.queue.remove(0);
                println!("Entity {gem_id} finished casting.");
            }
        }

        // === Sync Castbar Visuals ===
        for (&em_id, castbar) in self.em.castbars.iter_mut() {
            let Some(&gem_id) = self.em_gem_link.get(&em_id) else { continue; };
            let Some(queue) = self.gem.actionqueue.get(&gem_id) else { continue; };
            let Some(action) = queue.queue.first() else {
                castbar.cast_progress = 0.0;
                continue;
            };

            if action.action == Actions::CastingSpell {
                let time_total = action.time_action_takes.max(1) as f32;
                let time_remaining = action.time_remaining.min(action.time_action_takes) as f32;
                let progress = 1.0 - (time_remaining / time_total);
                castbar.cast_progress = progress.clamp(0.0, 1.0);
                castbar.icon_name = action.action_tag.clone(); // assuming tag is the spell texture name
            } else {
                castbar.cast_progress = 0.0;
            }
        }
    }
}
