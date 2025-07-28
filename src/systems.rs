use crate::game::*;

impl Game {
    pub fn s_debuffs(&mut self) {
        if self.state[0] == 1 { // miasma
            let enemy_id = self.gem.get_enemy();
            let next_id = self.gem.next_pid();
            self.gem.debuffbars.get_mut(&enemy_id.unwrap()).unwrap().debuffs.push(crate::g_properties::GPDebuff {
                id: (next_id),
                name: ("miasma".to_string()),
                total_duration: (4000),
                time_left: (4000),
                stacks: (1),
                pending_damage: 0.0 });

            println!("miasma applied");
            self.state[0] = 0;
        }

        let mut enemies = self.gem.get_all_enemies();
        for enemy in enemies {
            let mut dbb = self.gem.debuffbars.get_mut(&enemy);
            if dbb.is_none() { // skip if entity doesn't have a debuffbar
                continue;
            }
            if dbb.as_ref().unwrap().debuffs.is_empty() { // skip if debuffbar vec is empty
                continue;
            }

            // Remove expired debuffs
            dbb.as_mut().unwrap().debuffs.retain(|debuff| debuff.time_left > 0);

            for debuff in dbb.as_mut().unwrap().debuffs.iter_mut() {
                if let Some(spell) = crate::g_properties::get_spelldata_from_string(debuff.name.clone()) {
                    if let Some(e_stats) = self.gem.stats.get_mut(&enemy) {
                        let dt_sec = self.delta_time_ms as f32 / 1000.0;
                        debuff.pending_damage += spell.dps as f32 * dt_sec * spell.coefficient as f32;

                        let whole_damage = debuff.pending_damage.floor() as u32;
                        debuff.pending_damage -= whole_damage as f32;

                        if e_stats.health_curr > whole_damage {
                            e_stats.health_curr -= whole_damage;
                        } else {
                            e_stats.health_curr = 0;
                        }

                        println!("dt_sec: {}", dt_sec);
                        println!("damage: {}", whole_damage);
                        println!("health: {}", e_stats.health_curr);
                    }
                }

                debuff.time_left = debuff.time_left.saturating_sub(self.delta_time_ms);
            }

        }
    }
}