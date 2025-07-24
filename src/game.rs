use std::collections::HashMap;
use sfml::audio::listener::position;
use sfml::cpp::FBox;
use sfml::graphics::{Color, Font, Text, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable};
use sfml::graphics::{Texture, Sprite};
use sfml::window::{Event, Style};
use sfml::window::mouse::Button;

use std::fs;
use std::mem;
use serde::{Serialize, Deserialize};
use crate::{entities, properties::*};
use std::sync::LazyLock;

pub static BASE: Color    = Color::rgba(36, 41, 46, 255);
pub static LIGHTER: Color = Color::rgba(43, 49, 55, 255);
pub static ENCAPSULATION_REGIONS: Color  = Color::rgba(29, 33, 37, 255);
pub static BUTTON: Color  = Color::rgba(19, 81, 150, 255);
pub static BORDER: Color  = Color::rgba(24, 26, 28, 255);

#[derive(Debug,)]
pub struct Game {
    window: FBox<RenderWindow>,
    window_width: u32,
    window_height: u32,
    state: Vec<u32>, // see state definitions at the bottom of file to make sense of this.
    em: entities::EntityManager,
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

        Game { window, window_width: w_width, window_height: w_height, state: state_vec, em: entities::EntityManager::new(), fnt: font, textures: HashMap::new()}
    }

    pub fn init(&mut self) {
        self.load_textures("./src/assets/sprites");
        self.init_gui();
    }

pub fn init_gui(&mut self) {
    let scale_w = self.window_width as f32 / 1920.0;
    let scale_h = self.window_height as f32 / 1080.0;
    let scale = scale_w.min(scale_h).floor().max(1.0) as u32;
    let s = |x: u32| x * scale; // closure !

    // Run button
    let rbid = self.em.add_entity("run_button".to_string(), entities::EntityType::Button);
    self.em.add_component_to_entity(PropertiesEnum::rect, rbid);
    self.em.add_component_to_entity(PropertiesEnum::text, rbid);
    if let Some(rect) = self.em.rectangles.get_mut(&rbid) {
        rect.width  = s(300);
        rect.height = s(100);
        rect.x      = s(10);
        rect.y      = s(10);
        rect.r = 19; rect.g = 81; rect.b = 150;
        rect.draw = true;
        rect.strata = 5;
    }
    if let Some(text) = self.em.texts.get_mut(&rbid) {
        text.scale = scale;
        text.x = 50;
        text.y = 50;
        text.r = 255;
        text.g = 255;
        text.b = 255;
        text.draw = true;
        text.strata = 10;
    }    

    // "Textbox"
    let tbid = self.em.add_entity("textbox_encap".to_string(), entities::EntityType::Region);
    self.em.add_component_to_entity(PropertiesEnum::rect, tbid);
    if let Some(rect) = self.em.rectangles.get_mut(&tbid) {
        rect.width  = s(610);
        rect.height = s(800);
        rect.x      = s(10);
        rect.y      = s(120);
        rect.r = 29; rect.g = 33; rect.b = 37;
        rect.draw = true;
        rect.strata = 10;
    }

    // Landscape region
    let lseid = self.em.add_entity("landscape".to_string(), entities::EntityType::Region);
    self.em.add_component_to_entity(PropertiesEnum::rect, lseid);
    if let Some(rect) = self.em.rectangles.get_mut(&lseid) {
        rect.width  = s(1044);
        rect.height = s(532);
        rect.x      = s(1920 - 1044) - s(10);
        rect.y      = s(10);
        rect.draw = true;
        rect.r = 29; rect.g = 33; rect.b = 37;
        rect.draw = true;
        rect.strata = 5;
    }

    // Landscape sprite
    self.em.add_component_to_entity(PropertiesEnum::sprite, lseid);
    if let Some(spr) = self.em.sprites.get_mut(&lseid) {
        spr.scale = scale;
        spr.x = s(1920 - 1024) - s(20);
        spr.y = s(20);
        spr.sprite_name = "rainier_background_2".to_string();
        spr.draw = true;
        spr.strata = 10;
    }

    let alpeid = self.em.add_entity("Alpe".to_string(), entities::EntityType::Enemy);
    self.em.add_component_to_entity(PropertiesEnum::sprite, alpeid);
    if let Some(spr) = self.em.sprites.get_mut(&alpeid) {
        spr.scale = scale * 4;
        spr.x = 3000;
        spr.y = 420;
        spr.sprite_name = "Alpe".to_string();
        spr.draw = true;
        spr.strata = 15
    }

    let lockid = self.em.add_entity("warlock".to_string(), entities::EntityType::Player);
    self.em.add_component_to_entity(PropertiesEnum::sprite, lockid);
    if let Some(spr) = self.em.sprites.get_mut(&lockid) {
        spr.scale = scale * 4;
        spr.x = 2000;
        spr.y = 420;
        spr.sprite_name = "main_warlock".to_string();
        spr.draw = true;
        spr.strata = 15
    }

    // foreground overlay for landscape, where things will stand.
    let ls_overlay_id = self.em.add_entity("landscape_overlay".to_string(), entities::EntityType::Region);
    self.em.add_component_to_entity(PropertiesEnum::sprite, ls_overlay_id);
    if let Some(spr) = self.em.sprites.get_mut(&ls_overlay_id) {
        spr.scale = scale;
        spr.x = s(1920 - 1024) - s(20);
        spr.y = s(20);
        spr.sprite_name = "ground_overlay3".to_string();
        spr.draw = true;
        spr.strata =  20;
    }

    let player_info_region = self.em.add_entity("player_info_region".to_string(), entities::EntityType::Region);
    self.em.add_component_to_entity(PropertiesEnum::rect, player_info_region);
    if let Some(rect) = self.em.rectangles.get_mut(&player_info_region) {
        rect.width  = s(517);
        rect.height = s(200);
        rect.x      = s(1920 - 1044) - s(10);
        rect.y      = s(532 + 10 + 10);
        rect.draw = true;
        rect.r = 29; rect.g = 33; rect.b = 37;
        rect.draw = true;
        rect.strata = 10;
    }

    let enemy_info_region = self.em.add_entity("enemy_info_region".to_string(), entities::EntityType::Region);
    self.em.add_component_to_entity(PropertiesEnum::rect, enemy_info_region);
    if let Some(rect) = self.em.rectangles.get_mut(&enemy_info_region) {
        rect.width  = s(517);
        rect.height = s(200);
        rect.x      = s(1920 - 512) - s(10);
        rect.y      = s(532 + 10 + 10);
        rect.draw = true;
        rect.r = 29; rect.g = 33; rect.b = 37;
        rect.draw = true;
        rect.strata = 10;
    }


}


    pub fn run(&mut self) {
        while self.window.is_open() {
            while let Some(event) = self.window.poll_event() {
                match event {
                    Event::Closed => self.window.close(),
                    _ => {}
                }
            }

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
        }

        let mut draw_list: Vec<(u8, DrawableItem)> = Vec::new();

        for rect in self.em.rectangles.values() {
            if rect.draw {
                draw_list.push((rect.strata, DrawableItem::Rect(rect)));
            }
        }

        for text in self.em.texts.values() {
            if text.draw {
                draw_list.push((text.strata, DrawableItem::Text(text)));
            }
        }

        for sprite in self.em.sprites.values() {
            if sprite.draw {
                draw_list.push((sprite.strata, DrawableItem::Sprite(sprite)));
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
                    draw_rect.set_fill_color(Color::rgb(rect.r, rect.g, rect.b));
                    self.window.draw(&draw_rect);
                }
                DrawableItem::Text(text) => {
                    let mut draw_text = Text::new(&text.text, &self.fnt, 20);
                    draw_text.set_scale(text.scale as f32);
                    draw_text.set_position((text.x as f32, text.y as f32));
                    draw_text.set_fill_color(Color::rgb(text.r, text.g, text.b));
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

    fn create_sprite(&self, texture_key: String) -> Option<Sprite> {
        self.textures.get(&texture_key).map(|texture| Sprite::with_texture(&**texture))
    }

}