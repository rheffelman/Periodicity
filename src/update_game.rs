use std::{collections::HashMap, hash::Hash};
use serde::{Serialize, Deserialize};
use crate::game::*;
use crate::g_properties::*;

impl Game {
    pub fn update_game_main_entry(&mut self) {
        if let Some(player_id_gem) = self.gem.player_id {
            if let Some(queue) = self.gem.actionqueue.get_mut(&player_id_gem) {
                if let Some(current_action) = queue.queue.first_mut() {
                    let dt_ms = (self.delta_time * 1000.0) as u32;
                    current_action.time_remaining =
                        current_action.time_remaining.saturating_sub(dt_ms);

                    if current_action.time_remaining == 0 {
                        queue.queue.remove(0);
                    }
                }
            }
        }
    }
}
