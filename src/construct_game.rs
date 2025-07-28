use crate::g_properties::Allegiances;
use crate::g_properties::GPActionQueue;
use crate::g_properties::GPAllegiance;
use crate::g_properties::GPBuffBar;
use crate::g_properties::GPDebuff;
use crate::g_properties::GPMortality;
use crate::g_properties::GPStats;
use crate::g_properties::GPTarget;
use crate::game::*;
use crate::properties::*;
use std::fs;

impl Game {
    pub fn init_game(&mut self) {
        self.create_player();
        self.create_enemy();
    }

    fn create_player(&mut self) {
        let pgid = self.gem.add_entity(Some("player".to_string()));

        let next_id = self.gem.next_pid();
        self.gem.mortalities.insert(pgid, GPMortality {
            id: next_id,
            is_alive: true,
        });

        let next_id = self.gem.next_pid();
        self.gem.allegiances.insert(pgid, GPAllegiance {
            id: next_id,
            allegiance: Allegiances::Player,
        });

        let next_id = self.gem.next_pid();
        self.gem.stats.insert(pgid, GPStats {
            id: next_id,
            health_max: 100,
            health_curr: 80,
            chaos: 1,
            solidity: 1,
            vitality: 1,
            haste: 1,
            will: 1,
            volatility: 1,
        });

        let next_id = self.gem.next_pid();
        self.gem.targets.insert(pgid, GPTarget { 
            id: (next_id), 
            target_entity: (None)
        });

        let next_id = self.gem.next_pid();
        self.gem.buffbars.insert(pgid, GPBuffBar {
            id: next_id,
            buffs: Vec::new(),
        });

        let next_id = self.gem.next_pid();
        self.gem.debuffbars.insert(pgid, crate::g_properties::GPDebuffBar {
            id: next_id,
            debuffs: Vec::new(),
        });

        let next_id = self.gem.next_pid();
        self.gem.actionqueue.insert(pgid, GPActionQueue {
            id: next_id,
            queue: Vec::new(),
        });

        self.gem.player_id = Some(pgid);
        if let Some(em_player_id) = self.em.get_player_id() {
            self.em_gem_link.insert(em_player_id, pgid);
        }
    }

    fn create_enemy(&mut self) {
        let enemy_id = self.gem.add_entity(Some("alpine_terror".to_string()));

        let next_id = self.gem.next_pid();
        self.gem.mortalities.insert(enemy_id, GPMortality {
            id: next_id,
            is_alive: true,
        });

        let next_id = self.gem.next_pid();
        self.gem.allegiances.insert(enemy_id, GPAllegiance {
            id: next_id,
            allegiance: Allegiances::Enemy,
        });

        let next_id = self.gem.next_pid();
        self.gem.stats.insert(enemy_id, GPStats {
            id: next_id,
            health_max: 100,
            health_curr: 80,
            chaos: 1,
            solidity: 1,
            vitality: 1,
            haste: 1,
            will: 1,
            volatility: 1,
        });

        let next_id = self.gem.next_pid();
        self.gem.targets.insert(enemy_id, GPTarget { 
            id: (next_id), 
            target_entity: (None)
        });

        let next_id = self.gem.next_pid();
        self.gem.buffbars.insert(enemy_id, GPBuffBar {
            id: next_id,
            buffs: Vec::new(),
        });

        let next_id = self.gem.next_pid();
        self.gem.debuffbars.insert(enemy_id, crate::g_properties::GPDebuffBar {
            id: next_id,
            debuffs: Vec::new(),
        });

        let next_id = self.gem.next_pid();
        self.gem.actionqueue.insert(enemy_id, GPActionQueue {
            id: next_id,
            queue: Vec::new(),
        });
    }
}