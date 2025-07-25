use std::collections::HashMap;
use sfml::audio::listener::position;
use sfml::cpp::FBox;
use sfml::graphics::{Color, Font, Text, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable};
use sfml::graphics::{Texture, Sprite};
use sfml::window::{Event, Style};
use sfml::window::mouse::Button;
use std::time::Instant;
use std::fs;
use std::mem;
use serde::{Serialize, Deserialize};
use crate::{entities, properties::*};
use crate::{user_input::*};
use std::sync::LazyLock;
pub static BASE: Color    = Color::rgba(36, 41, 46, 255);
pub static LIGHTER: Color = Color::rgba(43, 49, 55, 255);
pub static ENCAPSULATION_REGIONS: Color  = Color::rgba(29, 33, 37, 255);
pub static BUTTON: Color  = Color::rgba(19, 81, 150, 255);
pub static BORDER: Color  = Color::rgba(24, 26, 28, 255);

#[derive(Debug,)]
pub struct Game {
    pub window: FBox<RenderWindow>,
    window_width: u32,
    window_height: u32,
    state: Vec<u32>, // see state definitions at the bottom of file to make sense of this.
    pub user_input_cache: Vec<u32>,
    pub input_index: usize,
    pub em: entities::EntityManager,
    fnt: FBox<Font>,
    textures: HashMap<String, FBox<Texture>>,
}

impl Game {
    pub fn new(w_width: u32, w_height: u32) -> Game {
        let mut window = RenderWindow::new(
            (w_width, w_height),
            "Periodicity",
            Style::CLOSE,
            &Default::default(),
        )
        .expect("Failed to create SFML RenderWindow");
        window.set_vertical_sync_enabled(true);

        let font = Font::from_file("./src/assets/lilex.ttf")
            .expect("Failed to load font");
        
        let one_mb_bytes: usize = 1024 * 1024;
        let u32_size: usize = mem::size_of::<u32>();
        let num_elements: usize = one_mb_bytes / u32_size;

        let state_vec: Vec<u32> = vec![0; num_elements];
        let user_input_vec: Vec<u32> = vec![0; num_elements];

        Game { window, window_width: w_width, window_height: w_height, state: state_vec, user_input_cache: user_input_vec, input_index: 0, em: entities::EntityManager::new(), fnt: font, textures: HashMap::new()}
    }

    pub fn init(&mut self) {
        self.fnt.set_smooth(true);
        self.spawn_player();
        self.load_textures("./src/assets/sprites");
        self.init_gui();
    }

    pub fn init_gui(&mut self) {
        let scale_w = self.window_width as f32 / 1920.0;
        let scale_h = self.window_height as f32 / 1080.0;
        let scale = scale_w.min(scale_h).floor().max(1.0) as u32;
        let s = |x: u32| x * scale; // closure !

        let rbid = self.em.add_entity("run_button".to_string(), entities::EntityType::Button);
        self.em.add_property_to_entity(PropertiesEnum::rect, rbid);
        self.em.add_property_to_entity(PropertiesEnum::text, rbid);
        if let Some(rect) = self.em.rectangles.get_mut(&rbid) {
            rect.width  = s(300);
            rect.height = s(100);
            rect.x      = s(10);
            rect.y      = s(10);
            rect.colors.fill = (19, 81, 150);
            rect.colors.outline = (24, 26, 28);
            rect.draw = true;
            rect.strata = 5;
        }
        if let Some(texts) = self.em.texts.get_mut(&rbid) {
            if let Some(text) = texts.get_mut(0) {
                text.scale = scale;
                text.x = 50;
                text.y = 50;
                text.colors.fill = (255, 255, 255);
                text.colors.outline = (0, 0, 0);
                text.draw = true;
                text.strata = 10;
            }
        }

        let tbid = self.em.add_entity("textbox_encap".to_string(), entities::EntityType::Region);
        self.em.add_property_to_entity(PropertiesEnum::rect, tbid);
        if let Some(rect) = self.em.rectangles.get_mut(&tbid) {
            rect.width  = s(610);
            rect.height = s(800);
            rect.x      = s(10);
            rect.y      = s(120);
            rect.colors.fill = (29, 33, 37);
            rect.colors.outline = (0, 0, 0);
            rect.draw = true;
            rect.strata = 10;
        }

        let lseid = self.em.add_entity("landscape".to_string(), entities::EntityType::Region);
        self.em.add_property_to_entity(PropertiesEnum::rect, lseid);
        if let Some(rect) = self.em.rectangles.get_mut(&lseid) {
            rect.width  = s(1044);
            rect.height = s(532);
            rect.x      = s(1920 - 1044) - s(10);
            rect.y      = s(10);
            rect.colors.fill = (29, 33, 37);
            rect.colors.outline = (0, 0, 0);
            rect.draw = true;
            rect.strata = 5;
        }

        self.em.add_property_to_entity(PropertiesEnum::sprite, lseid);
        if let Some(spr) = self.em.sprites.get_mut(&lseid) {
            spr.scale = scale;
            spr.x = s(1920 - 1024) - s(20);
            spr.y = s(20);
            spr.sprite_name = "rainier_background_2".to_string();
            spr.draw = true;
            spr.strata = 10;
        }

        let alpeid = self.em.add_entity("Alpe".to_string(), entities::EntityType::Enemy);
        self.em.add_property_to_entity(PropertiesEnum::sprite, alpeid);
        if let Some(spr) = self.em.sprites.get_mut(&alpeid) {
            spr.scale = scale * 4;
            spr.x = 3000;
            spr.y = 420;
            spr.sprite_name = "Alpe".to_string();
            spr.draw = true;
            spr.strata = 15;
        }

        let ls_overlay_id = self.em.add_entity("landscape_overlay".to_string(), entities::EntityType::Region);
        self.em.add_property_to_entity(PropertiesEnum::sprite, ls_overlay_id);
        if let Some(spr) = self.em.sprites.get_mut(&ls_overlay_id) {
            spr.scale = scale;
            spr.x = s(1920 - 1024) - s(20);
            spr.y = s(20);
            spr.sprite_name = "ground_overlay3".to_string();
            spr.draw = true;
            spr.strata = 20;
        }

        let enemy_info_region = self.em.add_entity("enemy_info_region".to_string(), entities::EntityType::Region);
        self.em.add_property_to_entity(PropertiesEnum::rect, enemy_info_region);
        if let Some(rect) = self.em.rectangles.get_mut(&enemy_info_region) {
            rect.width  = s(512);
            rect.height = s(200);
            rect.x      = s(1920 - 512) - s(10);
            rect.y      = s(532 + 10 + 10);
            rect.colors.fill = (29, 33, 37);
            rect.colors.outline = (0, 0, 0);
            rect.draw = true;
            rect.strata = 10;
        }

        self.em.add_property_to_entity(PropertiesEnum::text, enemy_info_region);
        if let Some(texts) = self.em.texts.get_mut(&enemy_info_region) {
            if let Some(text) = texts.get_mut(0) {
                text.scale = scale;
                text.x = s(1920 - 510) - s(10);
                text.y = s(532 + 10 + 10);
                text.colors.fill = (255, 255, 255);
                text.colors.outline = (0, 0, 0);
                text.draw = true;
                text.strata = 10;
                text.text = "Alpine Terror".to_string();
            }
        }

        let tooltip_region = self.em.add_entity("tooltip".to_string(), entities::EntityType::Region);
        self.em.add_property_to_entity(PropertiesEnum::rect, tooltip_region);
        if let Some(rect) = self.em.rectangles.get_mut(&tooltip_region) {
            rect.width  = s(517);
            rect.height = s(200);
            rect.x      = s(1920 - 1044) - s(10);
            rect.y      = s(532 + 250);
            rect.colors.fill = (29, 33, 37);
            rect.colors.outline = (0, 0, 0);
            rect.draw = true;
            rect.strata = 10;
        }
    }

    fn spawn_player(&mut self) {
        let scale_w = self.window_width as f32 / 1920.0;
        let scale_h = self.window_height as f32 / 1080.0;
        let scale = scale_w.min(scale_h).floor().max(1.0) as u32;
        let s = |x: u32| x * scale;

        let player_id = self.em.add_entity("player".to_string(), entities::EntityType::Player);
        self.em.add_property_to_entity(PropertiesEnum::sprite, player_id);
        if let Some(spr) = self.em.sprites.get_mut(&player_id) {
            spr.scale = scale * 4;
            spr.x = 2000;
            spr.y = 420;
            spr.sprite_name = "main_warlock".to_string();
            spr.draw = true;
            spr.strata = 15;
        }
        self.em.add_property_to_entity(PropertiesEnum::rect, player_id);
        if let Some(rect) = self.em.rectangles.get_mut(&player_id) {
            rect.width  = s(517);
            rect.height = s(200);
            rect.x      = s(1920 - 1044) - s(10);
            rect.y      = s(532 + 10 + 10);
            rect.colors.fill = (29, 33, 37);
            rect.colors.outline = (0, 0, 0);
            rect.draw = true;
            rect.strata = 10;
        }
        self.em.add_property_to_entity(PropertiesEnum::text, player_id);
        if let Some(texts) = self.em.texts.get_mut(&player_id) {
            if let Some(text) = texts.get_mut(0) {
                text.scale = scale;
                text.x = s(1920 - 1042) - s(10);
                text.y = s(532 + 10 + 10);
                text.colors.fill = (255, 255, 255);
                text.colors.outline = (0, 0, 0);
                text.draw = true;
                text.strata = 10;
                text.text = "Player Info".to_string();
            }
        }
        self.em.add_property_to_entity(PropertiesEnum::stat, player_id);
        if let Some(stat) = self.em.stats.get_mut(&player_id) {
            stat.chaos = 1;
            stat.haste = 1;
            stat.solidity = 1;
            stat.vitality = 1;
            stat.health_max = 100;
            stat.health_curr = 70;
        }
        self.em.add_property_to_entity(PropertiesEnum::healthbar, player_id);
        if let Some(hb) = self.em.healthbars.get_mut(&player_id) {
            hb.x = s(1920) - s(1047);
            hb.y = s(532 + 30 + 30);
            hb.width = s(502);
            hb.height = s(50);
            hb.draw = true;
            hb.strata = 30;

            hb.base_colors = ColorPair { fill: (64, 64, 64), outline: (0, 0, 0) };
            hb.inner_colors = ColorPair { fill: (255, 100, 100), outline: (0, 0, 0) };
        }
        self.em.add_property_to_entity(PropertiesEnum::castbar, player_id);
        if let Some(cb) = self.em.castbars.get_mut(&player_id) {
            cb.x = s(1920) - s(1047);
            cb.y = s(532 + 30 + 30 + 60);
            cb.width = s(452);
            cb.height = s(50);
            cb.draw = true;
            cb.strata = 30;
            cb.cast_progress = 0.8;

            cb.base_colors = ColorPair { fill: (64, 64, 64), outline: (0, 0, 0) };
            cb.inner_colors = ColorPair { fill: (100, 100, 255), outline: (0, 0, 0) };
        }
        self.em.add_property_to_entity(PropertiesEnum::state, player_id);
    }

    pub fn run(&mut self) {
        let start_time = Instant::now();
        let mut prev_progress = 0.0;
        
        while self.window.is_open() {
            self.cache_user_input();

            let elapsed = start_time.elapsed().as_secs_f32();
            let progress = (elapsed % 5.0) / 5.0;

            if progress < prev_progress {
                for e in self.em.entities.clone() {

                }
                println!("Castbar finished!");
            }

            for (_, castbar) in self.em.castbars.iter_mut() {
                castbar.cast_progress = progress;
            }

            prev_progress = progress;
            self.render();
        }
    }

    fn render(&mut self) {
        self.window.clear(entities::BASE);

        #[derive(Debug)]
        enum DrawableItem<'a> {
            Rect(&'a PRect),
            Text(&'a PText),
            Sprite(&'a PSprite),
            Healthbar(&'a PHealthbar),
            Castbar(&'a PCastbar),
        }

        let mut draw_list: Vec<(u8, DrawableItem)> = Vec::new();

        for rect in self.em.rectangles.values() {
            if rect.draw {
                draw_list.push((rect.strata, DrawableItem::Rect(rect)));
            }
        }

        for (_id, text_list) in self.em.texts.iter() {
            for text in text_list {
                if text.draw {
                    draw_list.push((text.strata, DrawableItem::Text(text)));
                }
            }
        }

        for sprite in self.em.sprites.values() {
            if sprite.draw {
                draw_list.push((sprite.strata, DrawableItem::Sprite(sprite)));
            }
        }

        for healthbar in self.em.healthbars.values() {
            if healthbar.draw {
                draw_list.push((healthbar.strata, DrawableItem::Healthbar(healthbar)));
            }
        }

        for castbar in self.em.castbars.values() {
            if castbar.draw {
                draw_list.push((castbar.strata, DrawableItem::Castbar(castbar)));
            }
        }

        // sort by strata
        draw_list.sort_by_key(|(strata, _)| *strata);

        // draw in order of strata
        for (_strata, item) in draw_list {
            match item {
                DrawableItem::Rect(rect) => {
                    let mut draw_rect = RectangleShape::new();
                    draw_rect.set_size((rect.width as f32, rect.height as f32));
                    draw_rect.set_position((rect.x as f32, rect.y as f32));
                    draw_rect.set_fill_color(Color::rgb(rect.colors.fill.0, rect.colors.fill.1, rect.colors.fill.2));
                    draw_rect.set_outline_color(Color::rgb(rect.colors.outline.0, rect.colors.outline.1, rect.colors.outline.2));
                    draw_rect.set_outline_thickness(2.0);
                    self.window.draw(&draw_rect);
                }
                DrawableItem::Text(text) => {
                    let mut draw_text = Text::new(&text.text, &self.fnt, 20);
                    draw_text.set_scale(text.scale as f32);
                    draw_text.set_position((text.x as f32, text.y as f32));
                    draw_text.set_fill_color(Color::rgb(text.colors.fill.0, text.colors.fill.1, text.colors.fill.2));
                    draw_text.set_outline_color(Color::rgb(text.colors.outline.0, text.colors.outline.1, text.colors.outline.2));
                    draw_text.set_outline_thickness(1.0);
                    self.window.draw(&draw_text);
                }
                DrawableItem::Sprite(sprite) => {
                    if let Some(texture) = self.textures.get(&sprite.sprite_name) {
                        let mut draw_sprite = Sprite::with_texture(&**texture);
                        draw_sprite.set_scale(sprite.scale as f32);
                        draw_sprite.set_position((sprite.x as f32, sprite.y as f32));
                        self.window.draw(&draw_sprite);
                    }
                }
                DrawableItem::Healthbar(healthbar) => {
                    let stat = self.em.stats.iter().find(|(id, _)| {
                        self.em.healthbars.get(id).map(|hb| hb == healthbar).unwrap_or(false)
                    }).map(|(_, s)| s);

                    let health_ratio = if let Some(stat) = stat {
                        stat.health_curr as f32 / stat.health_max.max(1) as f32
                    } 
                    else {
                        1.0
                    };
                    let mut base_rect = RectangleShape::new();
                    base_rect.set_size((healthbar.width as f32, healthbar.height as f32));
                    base_rect.set_position((healthbar.x as f32, healthbar.y as f32));
                    base_rect.set_fill_color(Color::rgb(healthbar.base_colors.fill.0, healthbar.base_colors.fill.1, healthbar.base_colors.fill.2));
                    base_rect.set_outline_color(Color::rgb(healthbar.base_colors.outline.0, healthbar.base_colors.outline.1, healthbar.base_colors.outline.2));
                    base_rect.set_outline_thickness(2.0);
                    self.window.draw(&base_rect);

                    let inner_width = (healthbar.width as f32 * health_ratio).max(0.0);
                    let mut inner_rect = RectangleShape::new();
                    inner_rect.set_size((inner_width, healthbar.height as f32));
                    inner_rect.set_position((healthbar.x as f32, healthbar.y as f32));
                    inner_rect.set_fill_color(Color::rgb(healthbar.inner_colors.fill.0, healthbar.inner_colors.fill.1, healthbar.inner_colors.fill.2));
                    inner_rect.set_outline_color(Color::rgb(healthbar.inner_colors.outline.0, healthbar.inner_colors.outline.1, healthbar.inner_colors.outline.2));
                    inner_rect.set_outline_thickness(0.0);
                    self.window.draw(&inner_rect);
                }
                DrawableItem::Castbar(castbar) => {
                    // Outer bar (background)
                    let mut base_rect = RectangleShape::new();
                    base_rect.set_size((castbar.width as f32, castbar.height as f32));
                    base_rect.set_position((castbar.x as f32, castbar.y as f32));
                    base_rect.set_fill_color(Color::rgb(castbar.base_colors.fill.0, castbar.base_colors.fill.1, castbar.base_colors.fill.2));
                    base_rect.set_outline_color(Color::rgb(castbar.base_colors.outline.0, castbar.base_colors.outline.1, castbar.base_colors.outline.2));
                    base_rect.set_outline_thickness(2.0);
                    self.window.draw(&base_rect);

                    // Foreground bar (progress)
                    let filled_width = (castbar.width as f32 * castbar.cast_progress.clamp(0.0, 1.0)).max(0.0);
                    let mut inner_rect = RectangleShape::new();
                    inner_rect.set_size((filled_width, castbar.height as f32));
                    inner_rect.set_position((castbar.x as f32, castbar.y as f32));
                    inner_rect.set_fill_color(Color::rgb(castbar.inner_colors.fill.0, castbar.inner_colors.fill.1, castbar.inner_colors.fill.2));
                    inner_rect.set_outline_color(Color::rgb(castbar.inner_colors.outline.0, castbar.inner_colors.outline.1, castbar.inner_colors.outline.2));
                    inner_rect.set_outline_thickness(0.0);
                    self.window.draw(&inner_rect);

                    if let Some(texture) = self.textures.get(&castbar.icon_name) {
                        let mut icon_sprite = Sprite::with_texture(&**texture);
                        let icon_size = castbar.height as f32;
                        let original_size = texture.size();
                        let scale_x = icon_size / original_size.x as f32;
                        let scale_y = icon_size / original_size.y as f32;
                        icon_sprite.set_scale((scale_x, scale_y));
                        let icon_x = castbar.x as f32 + castbar.width as f32;
                        let icon_y = castbar.y as f32;
                        icon_sprite.set_position((icon_x, icon_y));

                        self.window.draw(&icon_sprite);
                    }

                }
            }
        }

        self.window.display();
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

                            match Texture::from_file(path_str) {
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