use crate::{g_properties::GPAction, game::*};

impl Game {
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
            time_action_takes: 4000,
            time_remaining: 4000,
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
        self.anims.add_animation_instance(crate::animation::AnimatedSprite {
            texture_id: "Miasma_anim2".to_string(),
            frame_width: 64,
            frame_height: 64,
            total_frames: 12,
            current_frame: 0,
            frame_time: 0.2,
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
        println!("B button pressed");
        let b_button = self.em.get_prects_mut_by_tag("b_button");
        b_button.unwrap()[0].pressed = Some(true);

        let player_id = self.gem.player_id.unwrap();
        let player_target = self.gem.targets.get_mut(&player_id);

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
            time_action_takes: 4000,
            time_remaining: 4000,
            spell: Some(crate::g_properties::Spells::Infernum),
        };
        self.gem.actions.insert(player_id, a.clone());
        println!("Infernum added to action queue");
        self.gem.actionqueue.get_mut(&player_id).unwrap().queue.push(a);
    }

    fn handle_c_button(&mut self) {
        println!("C button pressed");
    }

    fn handle_d_button(&mut self) {
        println!("D button pressed");
    }
}