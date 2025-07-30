use crate::{g_properties::GPAction, game::*};

impl Game<'_> {
    pub fn branch_from_click(&mut self, action: crate::properties::ClickAction) {
        if action == crate::properties::ClickAction::RunButton {
            self.handle_run_button();
        }
        else if action == crate::properties::ClickAction::A {
            self.handle_a_button();
        }
        else if action == crate::properties::ClickAction::B {
            self.handle_b_button();
        }
        else if action == crate::properties::ClickAction::C {
            self.handle_c_button();
        }
        else if action == crate::properties::ClickAction::D {
            self.handle_d_button();
        }
        else if action == crate::properties::ClickAction::G {
            self.handle_g_button();
        }
        else if action == crate::properties::ClickAction::H {
            self.handle_h_button();
        }
    }

    fn handle_run_button(&mut self) {
        println!("Hello from handle_run_button!");
        let run_button = self.em.get_prects_mut_by_tag("run_button");
        run_button.unwrap()[0].pressed = Some(true);
    }

    fn handle_a_button(&mut self) {
        println!("A button pressed");

        let a_button = self.em.get_prects_mut_by_tag("a_button");
        a_button.unwrap()[0].pressed = Some(true);

        let player_id = self.gem.player_id.unwrap();
        let player_target = self.gem.targets.get_mut(&player_id);

        // tries to find a game entity that's not the player to set as the target.
        let other_entity = self.gem.gids
            .iter()
            .find(|pair| *(pair.0) != player_id)
            .map(|(_, entity)| entity);

        if let (Some(entity), Some(target)) = (other_entity, player_target) {
            println!("Found a non-player entity: {:?}", entity);
            target.target_entity = Some(entity.id);
        } else {
            println!("No non-player entity found or player_target not found.");
            return
        }

        let next_id = self.gem.next_pid();
        let a = GPAction {
            id: next_id,
            action: crate::g_properties::Actions::CastingSpell,
            action_tag: "miasma".to_string(),
            time_action_takes: 2000,
            time_remaining: 2000,
            spell: Some(crate::g_properties::Spells::Miasma),
        };

        self.gem.actions.insert(player_id, a.clone());
        println!("Miasma added to action queue");
        self.gem.actionqueue.get_mut(&player_id).unwrap().queue.push(a);

        let scale_w = WINDOW_WIDTH as f32 / 1920.0;
        let scale_h = WINDOW_HEIGHT as f32 / 1080.0;
        let scale = scale_w.min(scale_h).floor().max(1.0) as u32;
        let s = |x: u32| x * scale;

        self.anims.remove_sprite_by_texture("my_warlock");
        self.anims.remove_sprite_by_texture("Miasma_anim2");
        let action_time = self.gem.actionqueue
            .get(&self.gem.player_id.unwrap())
            .unwrap()
            .queue
            .first()
            .unwrap()
            .time_action_takes;
        let action_time_sec = action_time as f32 / 1000.0;

        let frame_time = action_time_sec / 12.0;

        self.anims.add_animation_instance(crate::animation::AnimatedSprite {
            texture_id: "Miasma_anim2".to_string(),
            frame_width: 64,
            frame_height: 64,
            total_frames: 12,
            current_frame: 0,
            frame_time: Some(frame_time),
            time_accumulator: 0.0,
            position: (s(1000) , s(207)),
            inanimate: false,
            strata: 20,
            desired_width: Some(s(256)),
            desired_height: Some(s(256)),
            play_once: true,
            finished: false,
            velocity: (0.0, 0.0),
            lifetime: None,
        });
    }

    fn handle_b_button(&mut self) {
        let player_id = self.gem.player_id.unwrap();
        let player_target = self.gem.targets.get_mut(&player_id);

        let b_button = self.em.get_prects_mut_by_tag("b_button");
        b_button.unwrap()[0].pressed = Some(true);
        
        let other_entity = self.gem.gids
            .iter()
            .find(|pair| *(pair.0) != player_id)
            .map(|(_, entity)| entity);

        if let (Some(entity), Some(target)) = (other_entity, player_target) {
            println!("Found a non-player entity: {:?}", entity);
            target.target_entity = Some(entity.id);
        } else {
            println!("No non-player entity found or player_target not found.");
            return
        }

        let next_id = self.gem.next_pid();
        let a = GPAction {
            id: next_id,
            action: crate::g_properties::Actions::CastingSpell,
            action_tag: "infernum".to_string(),
            time_action_takes: 2000,
            time_remaining: 2000,
            spell: Some(crate::g_properties::Spells::Infernum),
        };

        self.gem.actions.insert(player_id, a.clone());
        println!("Infernum added to action queue");
        self.gem.actionqueue.get_mut(&player_id).unwrap().queue.push(a);

        let scale_w = WINDOW_WIDTH as f32 / 1920.0;
        let scale_h = WINDOW_HEIGHT as f32 / 1080.0;
        let scale = scale_w.min(scale_h).floor().max(1.0) as u32;
        let s = |x: u32| x * scale;

        self.anims.remove_sprite_by_texture("my_warlock");
        self.anims.remove_sprite_by_texture("Miasma_anim2");
        let action_time = self.gem.actionqueue
            .get(&self.gem.player_id.unwrap())
            .unwrap()
            .queue
            .first()
            .unwrap()
            .time_action_takes;
        let action_time_sec = action_time as f32 / 1000.0;

        let frame_time = action_time_sec / 12.0;

        self.anims.add_animation_instance(crate::animation::AnimatedSprite {
            texture_id: "Miasma_anim2".to_string(),
            frame_width: 64,
            frame_height: 64,
            total_frames: 12,
            current_frame: 0,
            frame_time: Some(frame_time),
            time_accumulator: 0.0,
            position: (s(1000) , s(207)),
            inanimate: false,
            strata: 20,
            desired_width: Some(s(256)),
            desired_height: Some(s(256)),
            play_once: true,
            finished: false,
            velocity: (0.0, 0.0),
            lifetime: None,
        });
    }

    fn handle_c_button(&mut self) {
        println!("C button pressed");
        let c_button = self.em.get_prects_mut_by_tag("c_button");
        c_button.unwrap()[0].pressed = Some(true);
        let e_id = self.gem.get_entity_id_from_name("alpine_terror".to_string());
        if let Some(stats) = self.gem.stats.get_mut(&e_id) {
            stats.health_curr = stats.health_curr.saturating_sub(5);
        }
    }

    fn handle_d_button(&mut self) {
        println!("D button pressed");
    }

    fn handle_g_button(&mut self) {
        if let Some(g_button) = self.em.get_prects_mut_by_tag("g_button") {
            g_button[0].pressed = Some(true);
        }

        if self.state[2] == 1 {
            self.state[2] = 0;
            for i in 1..7 {
                let rect_tag = format!("encap_{}", i);
                let icon_tag = format!("encap_icon_{}", i);
                self.em.purge_entity_by_tag(&rect_tag);
                self.em.purge_entity_by_tag(&icon_tag);
                if crate::helpers::get_stat(i).is_some() {
                    self.anims.remove_sprite_by_texture(&crate::helpers::get_stat(i).unwrap());
                }
                
            }
            return;
        } else {
            self.state[2] = 1;
        }

        let scale_w = WINDOW_WIDTH as f32 / 1920.0;
        let scale_h = WINDOW_HEIGHT as f32 / 1080.0;
        let scale = scale_w.min(scale_h).floor().max(1.0) as u32;
        let s = |x: u32| x * scale;

        for i in 1..7 {
            let rect_tag = format!("encap_{}", i);
            let tbid = self.em.add_entity(Some(rect_tag.clone()));
            self.em.add_property_to_entity(crate::properties::PropertiesEnum::Rect, tbid);

            let color = if i == 1 { sfml::graphics::Color::rgb(29, 33, 37) } else { ALT_BASE };
            if let Some(rects) = self.em.rectangles.get_mut(&tbid) {
                if let Some(rect) = rects.get_mut(0) {
                    rect.width = s(610);
                    rect.height = s(120);
                    rect.x = s(10);
                    rect.y = s(120 * i);
                    rect.colors.fill = (color.r, color.g, color.b);
                    rect.colors.outline = Some((0, 0, 0));
                    rect.draw = true;
                    rect.strata = 10;
                }
            }

            // Add icon and text if stat exists
            if let Some(stat_name) = crate::helpers::get_stat(i) {
                // Get player stat value before mutable borrow of self.em
                let player_stat_value = self.get_player_stats(i).unwrap().to_string();

                // Add icon as its own entity with a trackable tag
                let icon_tag = format!("encap_icon_{}", i);
                let icon_eid = self.em.add_entity(Some(icon_tag));
                self.em.add_property_to_entity(crate::properties::PropertiesEnum::Text, icon_eid); // Also adds to EntityManager tracking

                // Add text label to the icon entity
                if let Some(texts) = self.em.get_ptexts_mut(icon_eid) {
                    if let Some(text) = texts.get_mut(0) {
                        text.text = stat_name.clone() + ": " + &player_stat_value;
                        text.x = s(140);
                        text.y = s(120 * i + 30);
                        text.scale = s(3);
                        text.colors.fill = crate::helpers::get_stat_color(i).unwrap_or((255, 255, 255));
                        text.colors.outline = Some((0, 0, 0));
                        text.draw = true;
                        text.strata = 21;
                    }
                }

                self.anims.add_animation_instance(crate::animation::AnimatedSprite {
                    texture_id: stat_name,
                    frame_width: 64,
                    frame_height: 64,
                    total_frames: 12,
                    current_frame: 0,
                    frame_time: None,
                    time_accumulator: 0.0,
                    position: (s(10), s(120 * i)),
                    inanimate: false,
                    strata: 20,
                    desired_width: Some(s(120)),
                    desired_height: Some(s(120)),
                    play_once: true,
                    finished: false,
                    velocity: (0.0, 0.0),
                    lifetime: None,
                });
            }
        }
    }


    fn handle_h_button(&mut self) {
        println!("H button pressed");
        let h_button = self.em.get_prects_mut_by_tag("h_button");
        h_button.unwrap()[0].pressed = Some(true);
    }
}