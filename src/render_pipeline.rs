use std::time::Instant;
use sfml::cpp::FBox;
use sfml::graphics::{Color, Font, RenderStates, RenderWindow, Texture};
use sfml::graphics::{Text, RectangleShape, Sprite, RenderTarget, Transformable, Shape};
use sfml::window::{Style};
use crate::g_properties::Actions;
use crate::{entities, properties::*};
use crate::game::*;
use crate::user_input::*;
use crate::helpers::*;

#[derive(Debug, Clone)]
enum DrawableItem {
    Rect(PRect),
    Text(PText),
    Healthbar(PHealthbar),
    //Castbar(PCastbar),
}

impl Game<'_> {
    pub fn render_main_entry(&mut self) {
        self.window.clear(BASE);

        let draw_list = self.render_construct_draw_list();

        for (_strata, item) in draw_list {
            self.dispatch_item(item);
        }
        self.render_player_castbar();
        self.render_tooltips();
        self.handle_sprites();
        self.render_xp_bar();

        for ft in &self.floating_texts {
            let mut text = Text::new(&ft.value, &self.gbfnt, ft.scale);
            text.set_position(ft.position);
            text.set_fill_color(ft.color);
            text.set_outline_color(ft.outline);
            text.set_outline_thickness(1.0);
            self.window.draw(&text);
        }
        self.window.display();
    }

    fn dispatch_item(&mut self, drawable: DrawableItem) {
        match drawable {
            DrawableItem::Rect(rect) => self.render_rect(&rect),
            DrawableItem::Text(text) => self.render_text(&text),
            DrawableItem::Healthbar(hb) => self.render_healthbar(&hb),
        }
    }

    fn render_construct_draw_list(&self) -> Vec<(u8, DrawableItem)> {
        let mut draw_list: Vec<(u8, DrawableItem)> = Vec::new();

        for rects in self.em.rectangles.values() {
            for rect in rects {
                if rect.draw {
                    draw_list.push((rect.strata, DrawableItem::Rect(rect.clone())));
                }
            }
        }

        for (_id, text_list) in &self.em.texts {
            for text in text_list {
                if text.draw {
                    draw_list.push((text.strata, DrawableItem::Text(text.clone())));
                }
            }
        }

        for healthbar in self.em.healthbars.values() {
            if healthbar.draw {
                draw_list.push((healthbar.strata, DrawableItem::Healthbar(healthbar.clone())));
            }
        }

        draw_list.sort_by_key(|(strata, _)| *strata);
        draw_list
    }

    fn handle_sprites(&mut self) {
        let sprs = self.anims.get_drawables();
        for (sprite, texture_id) in sprs {
            let is_alive = self
                .gem.texture_to_entity
                .get(&texture_id)
                .and_then(|id| self.gem.mortalities.get(id))
                .map(|m| m.is_alive)
                .unwrap_or(true);

            if is_alive {
                self.window.draw(&sprite);
            } else {
                let mut states = RenderStates::default();
                states.shader = Some(&self.desat_shader);
                self.window.draw_with_renderstates(&sprite, &states);
            }
        }
    }

    fn render_tooltips(&mut self) {
        let scale = get_scale();
        let s = |x: u32| scaled(scale, x);
        let known_tooltips = self.em.tooltip_data.clone();

        let mouse_x = self.user_input_cache[InputSlot::MouseX as usize] as i32;
        let mouse_y = self.user_input_cache[InputSlot::MouseY as usize] as i32;

        for (_, data) in known_tooltips {
            let x = data.x as i32;
            let y = data.y as i32;
            let w = data.width as i32;
            let h = data.height as i32;

            if mouse_x >= x && mouse_x <= x + w && mouse_y >= y && mouse_y <= y + h {
                let tooltip_x = s(1920 - 1044) - s(10);
                let tooltip_y = s(532 + 250);
                let tooltip_w = s(517);

                // === Header Text ===
                let mut header_text = Text::new(&data.header, &self.fnt, 60);
                let header_bounds = header_text.local_bounds();
                let header_x = tooltip_x as f32 + ((tooltip_w as f32 - header_bounds.width) / 2.0) - header_bounds.left;
                let header_y = tooltip_y as f32 + s(10) as f32;
                header_text.set_position((header_x, header_y));
                header_text.set_fill_color(EPIC);
                self.window.draw(&header_text);

                // === Body Text ===
                let body_str = crate::helpers::wrap_text(
                    &data.body,
                    &self.fnt,
                    40,
                    tooltip_w as f32 - 2.0 * s(10) as f32,
                );
                let mut body_text = Text::new(&body_str, &self.fnt, 40);
                let body_x = tooltip_x as f32 + s(10) as f32;
                let body_y = tooltip_y as f32 + s(80) as f32;
                body_text.set_position((body_x, body_y));
                body_text.set_fill_color(Color::WHITE);
                self.window.draw(&body_text);

                if let Some(icon_id) = &data.icon {
                    if let Some(texture) = self.anims.textures.get(icon_id) {
                        let tex_size = texture.size();
                        let icon_size = s(64);

                        let aspr = crate::animation::AnimatedSprite {
                            texture_id: icon_id.clone(),
                            frame_width: tex_size.x,
                            frame_height: tex_size.y,
                            total_frames: 1,
                            current_frame: 0,
                            frame_time: None,
                            time_accumulator: 0.0,
                            position: (tooltip_x as u32, tooltip_y as u32),
                            inanimate: true,
                            strata: 9999,
                            desired_width: Some(icon_size),
                            desired_height: Some(icon_size),
                            play_once: false,
                            finished: false,
                            velocity: (0.0, 0.0),
                            lifetime: None,
                        };

                        if let Some(sprite) = self.anims.get_drawable(&aspr) {
                            self.window.draw(&sprite);
                        }
                    }
                }
            }
        }
    }

    fn render_rect(&mut self, rect: &PRect) {
        let mut draw_rect = RectangleShape::new();
        draw_rect.set_size((rect.width as f32, rect.height as f32));
        draw_rect.set_position((rect.x as f32, rect.y as f32));

        let (fill, outline) = match (rect.pressed, rect.hovered) {
            (Some(true), _) => {
                if let Some(pc) = &rect.pressed_color {
                    (pc.fill, pc.outline)
                } else {
                    (rect.colors.fill, rect.colors.outline)
                }
            }
            (Some(false) | None, Some(true)) => {
                if let Some(hc) = &rect.hovered_color {
                    (hc.fill, hc.outline)
                } else {
                    (rect.colors.fill, rect.colors.outline)
                }
            }
            _ => (rect.colors.fill, rect.colors.outline),
        };

        draw_rect.set_fill_color(Color::rgb(fill.0, fill.1, fill.2));
        if outline.is_some() {
            draw_rect.set_outline_color(Color::rgb(outline.unwrap().0, outline.unwrap().1, outline.unwrap().2));
            draw_rect.set_outline_thickness(2.0);
        }
        
        self.window.draw(&draw_rect);
    }

    fn render_text(&mut self, text: &PText) {
        let mut draw_text = Text::new(&text.text, &self.gbfnt, text.scale * 20);
        draw_text.set_position((text.x as f32, text.y as f32));
        draw_text.set_fill_color(Color::rgb(text.colors.fill.0, text.colors.fill.1, text.colors.fill.2));
        if let Some(outline) = text.colors.outline {
            draw_text.set_outline_color(Color::rgb(outline.0, outline.1, outline.2));
            draw_text.set_outline_thickness(1.0);
        } else {
            draw_text.set_outline_thickness(0.0);
        }
        self.window.draw(&draw_text);
    }

    fn render_healthbar(&mut self, healthbar: &PHealthbar) {
        let health_ratio = if let Some(gem_id) = healthbar.gem_entity_id {
            if let Some(stats) = self.gem.stats.get(&gem_id) {
                stats.health_curr as f32 / stats.health_max.max(1) as f32
            } else {
                1.0
            }
        } else {
            1.0
        };

        let mut base_rect = RectangleShape::new();
        base_rect.set_size((healthbar.width as f32, healthbar.height as f32));
        base_rect.set_position((healthbar.x as f32, healthbar.y as f32));
        base_rect.set_fill_color(Color::rgb(
            healthbar.base_colors.fill.0,
            healthbar.base_colors.fill.1,
            healthbar.base_colors.fill.2,
        ));
        if healthbar.base_colors.outline.is_some() {
            base_rect.set_outline_color(Color::rgb(
                healthbar.base_colors.outline.unwrap().0,
                healthbar.base_colors.outline.unwrap().1,
                healthbar.base_colors.outline.unwrap().2,
            ));
            base_rect.set_outline_thickness(2.0);
        }


        self.window.draw(&base_rect);

        let inner_width = (healthbar.width as f32 * health_ratio).max(0.0);
        let mut inner_rect = RectangleShape::new();
        inner_rect.set_size((inner_width, healthbar.height as f32));
        inner_rect.set_position((healthbar.x as f32, healthbar.y as f32));
        inner_rect.set_fill_color(Color::rgb(
            healthbar.inner_colors.fill.0,
            healthbar.inner_colors.fill.1,
            healthbar.inner_colors.fill.2,
        ));
        if healthbar.inner_colors.outline.is_some() {
            inner_rect.set_outline_color(Color::rgb(
                healthbar.inner_colors.outline.unwrap().0,
                healthbar.inner_colors.outline.unwrap().1,
                healthbar.inner_colors.outline.unwrap().2,
            ));
            inner_rect.set_outline_thickness(0.0);
        }

        
        self.window.draw(&inner_rect);
    }

    fn render_player_castbar(&mut self) {
        let player_id_gem = self.gem.player_id.unwrap();
        let player_id_em = self.em.get_player_id().unwrap();

        let queue = self.gem.actionqueue.get_mut(&player_id_gem).unwrap();

        if queue.queue.is_empty() {
            return;
        }
        let current_action = queue.queue.first_mut().unwrap();

        let dt_ms = (self.delta_time * 1000.0) as u32;

        if current_action.time_remaining == 0 {
            queue.queue.remove(0);
            return;
        }

        let castbar = self.em.castbars.get(&player_id_em).unwrap();
        let spell = current_action.spell.as_ref().unwrap().clone();
        let spell_data = crate::g_properties::get_spell_data(spell.clone()).unwrap();
        let time_total = current_action.time_action_takes.max(1) as f32;
        let time_remaining = current_action.time_remaining.min(current_action.time_action_takes) as f32;
        let cast_progress = 1.0 - (time_remaining / time_total);
        let filled_width = (castbar.width as f32 * cast_progress).max(0.0);

        let mut base_rect = RectangleShape::new();
        base_rect.set_size((castbar.width as f32, castbar.height as f32));
        base_rect.set_position((castbar.x as f32, castbar.y as f32));
        base_rect.set_fill_color(Color::rgb(
            castbar.base_colors.fill.0,
            castbar.base_colors.fill.1,
            castbar.base_colors.fill.2,
        ));

        if castbar.base_colors.outline.is_some() {
            base_rect.set_outline_color(Color::rgb(
                castbar.base_colors.outline.unwrap().0,
                castbar.base_colors.outline.unwrap().1,
                castbar.base_colors.outline.unwrap().2,
            ));
            base_rect.set_outline_thickness(2.0);
        }

        self.window.draw(&base_rect);

        // draw fill
        let mut inner_rect = RectangleShape::new();
        inner_rect.set_size((filled_width, castbar.height as f32));
        inner_rect.set_position((castbar.x as f32, castbar.y as f32));
        inner_rect.set_fill_color(Color::rgb(
            spell_data.colors.fill.0,
            spell_data.colors.fill.1,
            spell_data.colors.fill.2,
        ));
        if castbar.inner_colors .outline.is_some() {
            inner_rect.set_outline_color(Color::rgb(
                castbar.inner_colors.outline.unwrap().0,
                castbar.inner_colors.outline.unwrap().1,
                castbar.inner_colors.outline.unwrap().2,
            ));
            inner_rect.set_outline_thickness(0.0);
        }


        self.window.draw(&inner_rect);

        // spell icon
        if let Some(texture) = self.anims.textures.get(&current_action.action_tag) {
            let tex_size = texture.size();
            let icon_size = castbar.height;

            // Create a temporary AnimatedSprite descriptor
            let aspr = crate::animation::AnimatedSprite {
                texture_id: current_action.action_tag.clone(),
                frame_width: tex_size.x,
                frame_height: tex_size.y,
                total_frames: 1,
                current_frame: 0,
                frame_time: None,
                time_accumulator: 0.0,
                position: (castbar.x + castbar.width, castbar.y),
                inanimate: true,
                strata: 9999, // rendered last
                desired_width: Some(icon_size),
                desired_height: Some(icon_size),
                play_once: false,
                finished: false,
                velocity: (0.0, 0.0),
                lifetime: None,
            };

            if let Some(sprite) = self.anims.get_drawable(&aspr) {
                self.window.draw(&sprite);
            }
        }
    }

    fn render_xp_bar(&mut self) {
        let scale_w = self.window_width as f32 / 1920.0;
        let scale_h = self.window_height as f32 / 1080.0;
        let scale = scale_w.min(scale_h).floor().max(1.0) as u32;
        let s = |x: u32| x * scale;

        let player_id = self.gem.player_id.unwrap();
        let nlxp = self.gem.levels.get(&player_id).unwrap().next_level_xp;
        let player_level = self.gem.levels.get(&player_id).unwrap().curr_level;
        let cxp = self.gem.levels.get(&player_id).unwrap().curr_xp;
        let inner_width_coef = nlxp / cxp;

        let mut base_rect = RectangleShape::new();
        base_rect.set_size((s(502) as f32, s(50) as f32));
        base_rect.set_position((s(10) as f32, s(930) as f32));
        base_rect.set_fill_color(ENCAPSULATION_REGIONS);
        base_rect.set_outline_color(Color::rgba(0, 0, 0, 255));
        base_rect.set_outline_thickness(2.0);
        self.window.draw(&base_rect);
        
        let mut inner_rect = RectangleShape::new();
        inner_rect.set_size((s((502) /  inner_width_coef) as f32, s(50) as f32));
        inner_rect.set_position((s(10) as f32, s(930) as f32));
        inner_rect.set_fill_color(XP_COLOR);
        self.window.draw(&inner_rect);

        let mut draw_text = Text::new(&player_level.to_string(), &self.gbfnt, s(50));
        draw_text.set_position((s(10 + 502 + 40) as f32, s(920) as f32));
        draw_text.set_fill_color(XP_COLOR);
        draw_text.set_outline_color(Color::BLACK);
        draw_text.set_outline_thickness(2.0);
        self.window.draw(&draw_text);
        
    }
}