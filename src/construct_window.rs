use crate::animation::AnimatedSprite;
use crate::game::*;
use crate::properties::*;
use std::fs;

impl Game {
    pub fn init_main_entry(&mut self) {
        self.init_game();
        self.fnt.set_smooth(true);
        self.create_run_button();
        self.create_side_buttons();
        self.spawn_player();
        self.anims.load_textures("./src/assets/sprites");
        //self.load_textures("./src/assets/sprites");
        self.init_gui();
        
    }

    fn create_run_button(&mut self) {
        let scale = get_scale();
        let s = |x: u32| x * scale;

        let button_eid = self.em.create_button(Some("run_button".to_string()));
        let mut rect_id: Option<u32> = None;

        if let Some(rects) = self.em.get_prects_mut(button_eid) {
            if let Some(rect) = rects.last_mut() {
                rect.x = s(10);
                rect.y = s(50);
                rect.width = s(200);
                rect.height = s(50);
                rect.colors = ColorPair::from_colors(BUTTON, MAIN_OUTLINE_CLR);
                rect.pressed_color = Some(ColorPair::from_colors(BUTTON_PRESSED, MAIN_OUTLINE_CLR));
                rect.hovered_color = Some(ColorPair::from_colors(BUTTON_HOVERED, MAIN_OUTLINE_CLR));
                rect.draw = true;
                rect.hovered = Some(false);
                rect.strata = 10;
                rect_id = Some(rect.id);
            }
        }

        if let Some(texts) = self.em.get_ptexts_mut(button_eid) {
            if let Some(text) = texts.last_mut() {
                text.scale = scale;
                text.x = s(15);
                text.y = s(50);
                text.colors = ColorPair::from_colors(MAIN_TEXT_CLR, MAIN_OUTLINE_CLR);
                text.draw = true;
                text.strata = 15;
            }
        }

        if let Some(tt) = self.em.get_tooltip_data_mut(button_eid) {
            tt.header = "Run Code Button".to_string();
            tt.body = "Runs the code that is currently placed in the text editor section. Be careful! Arbitrary code execution can be dangerous.".to_string();
            tt.x = s(10);
            tt.y = s(50);
            tt.width = s(200);
            tt.height = s(50);
            tt.icon = None;
        }

        if let (Some(id), Some(clickable)) = (rect_id, self.em.get_pclickable_mut(button_eid)) {
            clickable.clickable = true;
            clickable.action = ClickAction::RunButton;
            clickable.rect_reference_id = Some(id);
        }
    }

    fn create_side_buttons(&mut self) {
        let scale = get_scale();
        let s = |x: u32| x * scale;

        let run_x = s(10);
        let run_width = s(200);
        let run_y = s(50);
        let button_height = s(50);

        let spacing = s(10);
        let left_x = run_x + run_width + spacing;

        let right_x = if let Some(tb_rects) = self.em.get_prects_by_name("textbox_encap") {
            if let Some(tb) = tb_rects.last() {
                tb.x + tb.width
            } else {
                left_x + s(400)
            }
        } else {
            left_x + s(400)
        };

        let total_width = right_x.saturating_sub(left_x);
        let button_width = total_width / 4;

        let labels = ["A", "B", "C", "D"];
        let actions = [
            ClickAction::A,
            ClickAction::B,
            ClickAction::C,
            ClickAction::D,
        ];

        for (i, (label, action)) in labels.iter().zip(actions.iter()).enumerate() {
            let eid = self.em.create_button(Some(format!("{}_button", label.to_lowercase())));
            let button_x = left_x + i as u32 * button_width;

            if let Some(rects) = self.em.get_prects_mut(eid) {
                if let Some(rect) = rects.last_mut() {
                    rect.x = button_x;
                    rect.y = run_y;
                    rect.width = button_width;
                    rect.height = button_height;
                    rect.colors = ColorPair::from_colors(ALT_BUTTON, MAIN_OUTLINE_CLR);
                    rect.pressed_color = Some(ColorPair::from_colors(ALT_BUTTON_PRESSED, MAIN_OUTLINE_CLR));
                    rect.hovered_color = Some(ColorPair::from_colors(ALT_BUTTON_HOVERED, MAIN_OUTLINE_CLR));
                    rect.draw = true;
                    rect.hovered = Some(false);
                    rect.strata = 10;
                }
            }

            if let Some(texts) = self.em.get_ptexts_mut(eid) {
                if let Some(text) = texts.last_mut() {
                    text.scale = scale.saturating_sub(1).max(1);
                    text.x = button_x + s(5);
                    text.y = run_y;
                    text.text = label.to_string();
                    text.colors = ColorPair::from_colors(MAIN_TEXT_CLR, MAIN_OUTLINE_CLR);
                    text.draw = true;
                    text.strata = 15;
                }
            }

            if let Some(tt) = self.em.get_tooltip_data_mut(eid) {
                tt.header = label.to_string();
                tt.body = format!("{} button functionality.", label);
                tt.x = button_x;
                tt.y = run_y;
                tt.width = button_width;
                tt.height = button_height;
                tt.icon = None;
            }

            if let (Some(rect_id), Some(clickable)) = (self.em.get_prects_mut(eid).and_then(|r| r.last()).map(|r| r.id), self.em.get_pclickable_mut(eid)) {
                clickable.clickable = true;
                clickable.action = action.clone();
                clickable.rect_reference_id = Some(rect_id);
            }
        }
    }

    fn init_gui(&mut self) {
        let scale_w = self.window_width as f32 / 1920.0;
        let scale_h = self.window_height as f32 / 1080.0;
        let scale = scale_w.min(scale_h).floor().max(1.0) as u32;
        let s = |x: u32| x * scale;

        // textbox encapsulation region
        let tbid = self.em.add_entity(Some("textbox_encap".to_string()));
        self.em.add_property_to_entity(PropertiesEnum::Rect, tbid);
        if let Some(rects) = self.em.rectangles.get_mut(&tbid) {
            if let Some(rect) = rects.get_mut(0) {
                rect.width = s(610);
                rect.height = s(800);
                rect.x = s(10);
                rect.y = s(120);
                rect.colors.fill = (29, 33, 37);
                rect.colors.outline = (0, 0, 0);
                rect.draw = true;
                rect.strata = 10;
            }
        }

        // landscape encapsulation region
        let lseid = self.em.add_entity(Some("landscape".to_string()));
        self.em.add_property_to_entity(PropertiesEnum::Rect, lseid);
        if let Some(rects) = self.em.rectangles.get_mut(&lseid) {
            if let Some(rect) = rects.get_mut(0) {
                rect.width = s(1044);
                rect.height = s(532);
                rect.x = s(1920 - 1044) - s(10);
                rect.y = s(10);
                rect.colors.fill = (29, 33, 37);
                rect.colors.outline = (0, 0, 0);
                rect.draw = true;
                rect.strata = 5;
            }
        }

        // forest background sprite
        self.anims.add_animation_instance(AnimatedSprite {
            texture_id: "rainier_background_2".to_string(),
            frame_width: 1024,
            frame_height: 512,
            total_frames: 1,
            current_frame: 0,
            frame_time: 9999.0,
            time_accumulator: 0.0,
            position: (s(1920 - 1044) , s(20)),
            inanimate: true,
            strata: 5,
            desired_width: Some(s(1024)),
            desired_height: Some(s(512)),
            play_once: false,
            finished: false,
            velocity: (0.0, 0.0),
            lifetime: None,
        });

        // alpine terror sprite
        self.anims.add_animation_instance(AnimatedSprite {
            texture_id: "Alpe".to_string(),
            frame_width: 64,
            frame_height: 64,
            total_frames: 1,
            current_frame: 0,
            frame_time: 9999.0,
            time_accumulator: 0.0,
            position: (s(1500) , s(207)),
            inanimate: true,
            strata: 10,
            desired_width: Some(s(256)),
            desired_height: Some(s(256)),
            play_once: false,
            finished: true,
            velocity: (0.0, 0.0),
            lifetime: None,
        });

        // ground overlay sprite
        self.anims.add_animation_instance(AnimatedSprite {
            texture_id: "ground_overlay3".to_string(),
            frame_width: 1024,
            frame_height: 512,
            total_frames: 1,
            current_frame: 0,
            frame_time: 9999.0,
            time_accumulator: 0.0,
            position: (s(1920 - 1044) , s(20)),
            inanimate: true,
            strata: 10,
            desired_width: Some(s(1024)),
            desired_height: Some(s(512)),
            play_once: false,
            finished: true,
            velocity: (0.0, 0.0),
            lifetime: None,
        });

        // enemy info region
        let enemy_info_region = self.em.add_entity(Some("enemy_info_region".to_string()));
        self.em.add_property_to_entity(PropertiesEnum::Rect, enemy_info_region);
        if let Some(rects) = self.em.rectangles.get_mut(&enemy_info_region) {
            if let Some(rect) = rects.get_mut(0) {
                rect.width = s(512);
                rect.height = s(200);
                rect.x = s(1920 - 512) - s(10);
                rect.y = s(532 + 20);
                rect.colors.fill = (29, 33, 37);
                rect.colors.outline = (0, 0, 0);
                rect.draw = true;
                rect.strata = 10;
            }
        }

        // enemy info region text
        self.em.add_property_to_entity(PropertiesEnum::Text, enemy_info_region);
        if let Some(texts) = self.em.texts.get_mut(&enemy_info_region) {
            if let Some(text) = texts.get_mut(0) {
                text.scale = scale;
                text.x = s(1920 - 510) - s(10);
                text.y = s(532 + 20);
                text.colors.fill = (255, 255, 255);
                text.colors.outline = (0, 0, 0);
                text.draw = true;
                text.strata = 10;
                text.text = "Alpine Terror".to_string();
            }
        }

        // enemy healthbar
        self.em.add_property_to_entity(PropertiesEnum::Healthbar, enemy_info_region);
        if let Some(hb) = self.em.get_phealthbar_mut(enemy_info_region) {
            hb.x = s(1920) - s(517);
            hb.y = s(532 + 60);
            hb.width = s(502);
            hb.height = s(50);
            hb.draw = true;
            hb.strata = 30;
            hb.base_colors = ColorPair {
                fill: (64, 64, 64),
                outline: (0, 0, 0),
            };
            hb.inner_colors = ColorPair {
                fill: (255, 100, 100),
                outline: (0, 0, 0),
            };
            hb.gem_entity_id = Some(self.gem.get_entity_id_from_name("alpine_terror".to_string()));
        }

        // tooltip region
        let tooltip_region = self.em.add_entity(Some("tooltip".to_string()));
        self.em.add_property_to_entity(PropertiesEnum::Rect, tooltip_region);
        if let Some(rects) = self.em.rectangles.get_mut(&tooltip_region) {
            if let Some(rect) = rects.get_mut(0) {
                rect.width = s(517);
                rect.height = s(200);
                rect.x = s(1920 - 1044) - s(10);
                rect.y = s(532 + 250);
                rect.colors.fill = (29, 33, 37);
                rect.colors.outline = (0, 0, 0);
                rect.draw = true;
                rect.strata = 10;
            }
        }
    }

    pub fn spawn_player(&mut self) {
        let scale_w = WINDOW_WIDTH as f32 / 1920.0;
        let scale_h = WINDOW_HEIGHT as f32 / 1080.0;
        let scale = scale_w.min(scale_h).floor().max(1.0) as u32;
        let s = |x: u32| x * scale;

        let player_id = self.em.add_entity(Some("player".to_string()));

        // player sprite
        self.anims.add_animation_instance(AnimatedSprite {
            texture_id: "my_warlock".to_string(),
            frame_width: 64,
            frame_height: 64,
            total_frames: 1,
            current_frame: 0,
            frame_time: 9999.0,
            time_accumulator: 0.0,
            position: (s(1000) , s(207)),
            inanimate: true,
            strata: 10,
            desired_width: Some(s(256)),
            desired_height: Some(s(256)),
            play_once: true,
            finished: false,
            velocity: (0.0, 0.0),
            lifetime: None,
        });

        // player info background encapsulation region
        self.em.add_property_to_entity(PropertiesEnum::Rect, player_id);
        if let Some(rects) = self.em.get_prects_mut(player_id) {
            if let Some(rect) = rects.first_mut() {
                rect.width = s(517);
                rect.height = s(200);
                rect.x = s(1920 - 1044) - s(10);
                rect.y = s(532 + 20);
                rect.colors = ColorPair {fill: (29, 33, 37), outline: (0, 0, 0)};
                rect.draw = true;
                rect.strata = 10;
            }
        }

        // player info header text
        self.em.add_property_to_entity(PropertiesEnum::Text, player_id);
        if let Some(texts) = self.em.get_ptexts_mut(player_id) {
            if let Some(text) = texts.first_mut() {
                text.text = "Player Info".into();
                text.scale = scale;
                text.x = s(1920 - 1042) - s(10);
                text.y = s(532 + 20);
                text.colors = ColorPair {fill: (255, 255, 255), outline: (0, 0, 0)};
                text.draw = true;
                text.strata = 10;
            }
        }

        // === HEALTHBAR ===
        self.em.add_property_to_entity(PropertiesEnum::Healthbar, player_id);
        if let Some(hb) = self.em.get_phealthbar_mut(player_id) {
            hb.x = s(1920) - s(1047);
            hb.y = s(532 + 60);
            hb.width = s(502);
            hb.height = s(50);
            hb.draw = true;
            hb.strata = 30;
            hb.base_colors = ColorPair {
                fill: (64, 64, 64),
                outline: (0, 0, 0),
            };
            hb.inner_colors = ColorPair {
                fill: (255, 100, 100),
                outline: (0, 0, 0),
            };
            hb.gem_entity_id = Some(self.gem.get_entity_id_from_name("player".to_string()));
        }

        // === CASTBAR ===
        self.em.add_property_to_entity(PropertiesEnum::Castbar, player_id);
        if let Some(cb) = self.em.get_pcastbar_mut(player_id) {
            cb.x = s(1920) - s(1047);
            cb.y = s(532 + 120);
            cb.width = s(452);
            cb.height = s(50);
            cb.cast_progress = 0.8;
            cb.draw = true;
            cb.strata = 30;
            cb.base_colors = ColorPair {
                fill: (64, 64, 64),
                outline: (0, 0, 0),
            };
            cb.inner_colors = ColorPair {
                fill: (100, 100, 255),
                outline: (0, 0, 0),
            };
            cb.icon_name = "miasma".into();
        }

        // === TOOLTIP ===
        self.em.add_property_to_entity(PropertiesEnum::TooltipData, player_id);
        if let Some(tt) = self.em.get_tooltip_data_mut(player_id) {
            tt.header = "Miasma".into();
            tt.body = "    A contagious metaphysical impurity. \nSpreads to any nearby enemies each time it \ndeals damage, haste does not affect its \ntickrate.".into();
            tt.x = s(1920) - s(1047);
            tt.y = s(532 + 120);
            tt.width = s(502);
            tt.height = s(50);
            tt.icon = Some("miasma".into());
        }

        // === STATE ===
        self.em.add_property_to_entity(PropertiesEnum::State, player_id);
    }

    fn load_textures(&mut self, folder_path: &str) {
        let entries = fs::read_dir(folder_path)
            .expect("Failed to read sprites folder");

        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext == "png" || ext == "jpg" || ext == "jpeg" {
                            let path_str = path.to_str().unwrap();
                            let file_name = path.file_stem().unwrap().to_str().unwrap().to_string();

                            match sfml::graphics::Texture::from_file(path_str) {
                                Ok(texture) => {
                                    self.textures.insert(file_name, texture);
                                    println!("Loaded texture: {}", path_str);
                                }
                                Err(err) => {
                                    eprintln!("Failed to load texture {}: {:?}", path_str, err);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
