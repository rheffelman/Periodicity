use std::time::Instant;
use sfml::cpp::FBox;
use sfml::graphics::{Color, Font, Texture, RenderWindow};
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
    Sprite(PSprite),
    Healthbar(PHealthbar),
    Castbar(PCastbar),
}

impl Game {

    pub fn render_main_entry(&mut self) {
        self.window.clear(BASE);

        let draw_list = self.render_construct_draw_list();

        for (_strata, item) in draw_list {
            self.dispatch_item(item);
        }
        self.render_tooltips();
        self.window.display();
    }

    fn dispatch_item(&mut self, drawable: DrawableItem) {
        match drawable {
            DrawableItem::Rect(rect) => self.render_rect(&rect),
            DrawableItem::Text(text) => self.render_text(&text),
            DrawableItem::Sprite(sprite) => self.render_sprite(&sprite),
            DrawableItem::Healthbar(hb) => self.render_healthbar(&hb),
            DrawableItem::Castbar(cb) => self.render_castbar(&cb),
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

        for sprite in self.em.sprites.values() {
            if sprite.draw {
                draw_list.push((sprite.strata, DrawableItem::Sprite(sprite.clone())));
            }
        }

        for healthbar in self.em.healthbars.values() {
            if healthbar.draw {
                draw_list.push((healthbar.strata, DrawableItem::Healthbar(healthbar.clone())));
            }
        }

        for castbar in self.em.castbars.values() {
            if castbar.draw {
                draw_list.push((castbar.strata, DrawableItem::Castbar(castbar.clone())));
            }
        }

        draw_list.sort_by_key(|(strata, _)| *strata);
        draw_list
    }

    fn render_tooltips(&mut self) {
        let scale = get_scale();
        let s = |x: u32| scaled(scale, x);
        let known_tooltips = self.em.tooltip_data.clone();

        let mouse_x = self.user_input_cache[InputSlot::MouseX as usize] as i32;
        let mouse_y = self.user_input_cache[InputSlot::MouseY as usize] as i32;

        for tooltip in known_tooltips {
            let x = tooltip.1.x as i32;
            let y = tooltip.1.y as i32;
            let w = tooltip.1.width as i32;
            let h = tooltip.1.height as i32;

            if mouse_x >= x && mouse_x <= x + w && mouse_y >= y && mouse_y <= y + h {
                let tooltip_x = s(1920 - 1044) - s(10);
                let tooltip_y = s(532 + 250);
                let tooltip_w = s(517);

                let mut header_text = Text::new(&tooltip.1.header, &self.fnt, 60);
                let header_bounds = header_text.local_bounds();
                let header_x = tooltip_x as f32 + ((tooltip_w as f32 - header_bounds.width) / 2.0) - header_bounds.left;
                let header_y = tooltip_y as f32 + s(10) as f32;
                header_text.set_position((header_x, header_y));
                header_text.set_fill_color(EPIC);

                let body_str = crate::helpers::wrap_text(
                    &tooltip.1.body,
                    &self.fnt,
                    40,
                    tooltip_w as f32 - 2.0 * s(10) as f32,
                );
                let mut body_text = Text::new(&body_str, &self.fnt, 40);
                let body_x = tooltip_x as f32 + s(10) as f32;
                let body_y = tooltip_y as f32 + s(80) as f32;
                body_text.set_position((body_x, body_y));
                body_text.set_fill_color(Color::WHITE);

                self.window.draw(&header_text);
                self.window.draw(&body_text);

                if let Some(icon_name) = &tooltip.1.icon {
                    if let Some(texture) = self.textures.get(icon_name) {
                        let mut icon_sprite = sfml::graphics::Sprite::with_texture(&**texture);

                        let desired_size = s(64) as f32;
                        let tex_size = texture.size();
                        let scale_x = desired_size / tex_size.x as f32;
                        let scale_y = desired_size / tex_size.y as f32;
                        icon_sprite.set_scale((scale_x, scale_y));

                        let icon_x = tooltip_x as f32 + s(10) as f32;
                        let icon_y = tooltip_y as f32 + s(10) as f32;
                        icon_sprite.set_position((icon_x, icon_y));

                        self.window.draw(&icon_sprite);
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
        draw_rect.set_outline_color(Color::rgb(outline.0, outline.1, outline.2));
        draw_rect.set_outline_thickness(2.0);
        self.window.draw(&draw_rect);
    }

    fn render_text(&mut self, text: &PText) {
        let mut draw_text = Text::new(&text.text, &self.fnt, text.scale * 20);
        //draw_text.set_scale(text.scale as f32);
        draw_text.set_position((text.x as f32, text.y as f32));
        draw_text.set_fill_color(Color::rgb(text.colors.fill.0, text.colors.fill.1, text.colors.fill.2));
        draw_text.set_outline_color(Color::rgb(text.colors.outline.0, text.colors.outline.1, text.colors.outline.2));
        draw_text.set_outline_thickness(1.0);
        self.window.draw(&draw_text);
    }

    fn render_sprite(&mut self, sprite: &PSprite) {
        if let Some(texture) = self.textures.get(&sprite.sprite_name) {
            let mut draw_sprite = Sprite::with_texture(&**texture);
            draw_sprite.set_position((sprite.x as f32, sprite.y as f32));
            draw_sprite.set_scale(sprite.scale as f32);

            if let Some(anim_name) = &sprite.anim_name {
                self.animator.apply_to_sprite(anim_name, &mut draw_sprite);
            }

            self.window.draw(&draw_sprite);
        }
    }

    fn render_healthbar(&mut self, healthbar: &PHealthbar) {
        let em_entity_id = self.em.healthbars.iter()
            .find_map(|(id, hb)| if hb == healthbar { Some(*id) } else { None });

        let health_ratio = if let Some(em_id) = em_entity_id {
            if let Some(&gem_id) = self.em_gem_link.get(&em_id) {
                if let Some(stats) = self.gem.stats.get(&gem_id) {
                    stats.health_curr as f32 / stats.health_max.max(1) as f32
                } else {
                    1.0
                }
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
        base_rect.set_outline_color(Color::rgb(
            healthbar.base_colors.outline.0,
            healthbar.base_colors.outline.1,
            healthbar.base_colors.outline.2,
        ));
        base_rect.set_outline_thickness(2.0);
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
        inner_rect.set_outline_color(Color::rgb(
            healthbar.inner_colors.outline.0,
            healthbar.inner_colors.outline.1,
            healthbar.inner_colors.outline.2,
        ));
        inner_rect.set_outline_thickness(0.0);
        self.window.draw(&inner_rect);
    }

    fn render_castbar(&mut self, castbar: &PCastbar) {
        let em_entity_id = self.em.castbars.iter()
            .find_map(|(id, cb)| if cb == castbar { Some(*id) } else { None });
        let Some(em_id) = em_entity_id else { return; };
        let Some(&gem_id) = self.em_gem_link.get(&em_id) else { return; };
        let Some(queue) = self.gem.actionqueue.get(&gem_id) else { return; };
        let Some(current_action) = queue.queue.first() else { return; };

        if current_action.action != Actions::CastingSpell {
            return;
        }

        // castbar fill ratio
        let spell = current_action.spell.as_ref().unwrap().clone();
        let spell_data = crate::g_properties::get_spell_data(spell.clone()).unwrap();
        let time_total = current_action.time_action_takes.max(1) as f32;
        let time_remaining = current_action.time_remaining.min(current_action.time_action_takes) as f32;
        let cast_progress = 1.0 - (time_remaining / time_total);
        let filled_width = (castbar.width as f32 * cast_progress).max(0.0);

        // draw background
        let mut base_rect = RectangleShape::new();
        base_rect.set_size((castbar.width as f32, castbar.height as f32));
        base_rect.set_position((castbar.x as f32, castbar.y as f32));
        base_rect.set_fill_color(Color::rgb(
            castbar.base_colors.fill.0,
            castbar.base_colors.fill.1,
            castbar.base_colors.fill.2,
        ));
        base_rect.set_outline_color(Color::rgb(
            castbar.base_colors.outline.0,
            castbar.base_colors.outline.1,
            castbar.base_colors.outline.2,
        ));
        base_rect.set_outline_thickness(2.0);
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
        inner_rect.set_outline_color(Color::rgb(
            castbar.inner_colors.outline.0,
            castbar.inner_colors.outline.1,
            castbar.inner_colors.outline.2,
        ));
        inner_rect.set_outline_thickness(0.0);
        self.window.draw(&inner_rect);

        // spell icon
        if let Some(texture) = self.textures.get(&current_action.action_tag) {
            let mut icon_sprite = Sprite::with_texture(&**texture);

            let icon_size = castbar.height as f32;
            let tex_size = texture.size();
            let scale_x = icon_size / tex_size.x as f32;
            let scale_y = icon_size / tex_size.y as f32;
            icon_sprite.set_scale((scale_x, scale_y));

            let icon_x = castbar.x as f32 + castbar.width as f32;
            let icon_y = castbar.y as f32;
            icon_sprite.set_position((icon_x, icon_y));

            self.window.draw(&icon_sprite);
        }
    }
}