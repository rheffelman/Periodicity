use std::{collections::HashMap, hash::Hash};
use serde::{Serialize, Deserialize};
use crate::game::*;
use crate::g_properties::*;

impl Game<'_> {
    pub fn update_game_main_entry(&mut self) {

        for ft in &mut self.floating_texts {
            ft.position.0 += ft.velocity.0 * self.delta_time;
            ft.position.1 += ft.velocity.1 * self.delta_time;
            ft.lifetime -= self.delta_time;
        }

        self.floating_texts.retain(|ft| ft.lifetime > 0.0);
        
        let scale_w = WINDOW_WIDTH as f32 / 1920.0;
        let scale_h = WINDOW_HEIGHT as f32 / 1080.0;
        let scale = scale_w.min(scale_h).floor().max(1.0) as u32;
        let s = |x: u32| x * scale;
        if let Some(player_id_gem) = self.gem.player_id {
            if let Some(queue) = self.gem.actionqueue.get_mut(&player_id_gem) {
                if let Some(current_action) = queue.queue.first_mut() {
                    let dt_ms = (self.delta_time * 1000.0) as u32;
                    current_action.time_remaining =
                        current_action.time_remaining.saturating_sub(dt_ms);

                    if current_action.time_remaining == 0 {
                        if current_action.action_tag == "miasma" {
                            self.state[0] = 1;
                        }
                        else if current_action.action_tag == "infernum" {
                            self.state[1] = 1;
                        }
                        queue.queue.remove(0);
                    }
                }
            }
        }
        if !self.miasma_has_spawned {
            if let Some(sprite) = self.anims.active.iter().find(|s| s.texture_id == "Miasma_anim2") {
                if sprite.current_frame == sprite.total_frames - 2 && sprite.play_once {
                    self.miasma_has_spawned = true;
                    self.anims.add_animation_instance(crate::animation::AnimatedSprite {
                        texture_id: "miasma_proj_anim2".to_string(),
                        frame_width: 64,
                        frame_height: 64,
                        total_frames: 2,
                        current_frame: 0,
                        frame_time: Some(0.2),
                        time_accumulator: 0.0,
                        position: (s(1050), s(250)),
                        inanimate: false,
                        strata: 50,
                        desired_width: Some(s(256)),
                        desired_height: Some(s(256)),
                        play_once: false,
                        finished: false,
                        velocity: (600.0, 0.0),
                        lifetime: Some(1.2),
                        associated_g_entity: None,
                    });
                }
            }
        }

        if !(self.anims.active.iter().any(|s| s.texture_id == "miasma_proj_anim2")) {
            self.miasma_has_spawned = false;
        }
    }
}
