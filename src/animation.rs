use std::collections::HashMap;
use std::hash::Hash;
use sfml::graphics::*;
use sfml::cpp::*;
use std::fs;

#[derive(Debug)]
pub struct AnimatedSprite {
    pub texture_id: String,
    pub frame_width: u32,
    pub frame_height: u32,
    pub total_frames: u32,
    pub current_frame: u32,
    pub frame_time: Option<f32>,
    pub time_accumulator: f32,
    pub position: (u32, u32),
    pub inanimate: bool,
    pub strata: u32,
    pub desired_width: Option<u32>,
    pub desired_height: Option<u32>,
    pub play_once: bool,
    pub finished: bool,
    pub velocity: (f32, f32),
    pub lifetime: Option<f32>,
    pub associated_g_entity: Option<u32>,
    //pub shader: Option<Box<Shader>>,
}

#[derive(Debug)]
pub struct Animation {
    pub textures: HashMap<String, FBox<Texture>>,
    pub active: Vec<AnimatedSprite>,
}

impl Animation {
    pub fn new() -> Self {
        Animation {
            textures: HashMap::new(),
            active: Vec::new(),
        }
    }

    pub fn load_texture(&mut self, id: &str, filepath: &str) {
        let tex = Texture::from_file(filepath).expect("Failed to load texture");
        self.textures.insert(id.to_string(), tex);
    }

    pub fn add_animation_instance(&mut self, sprite: AnimatedSprite) {
        self.active.push(sprite);
    }

    pub fn remove_sprite_by_texture(&mut self, texture_id: &str) {
        self.active.retain(|sprite| sprite.texture_id != texture_id);
    }

    // called every frame in game.run()
    pub fn update(&mut self, dt: f32) {
        for sprite in &mut self.active {
            // kill dead animations
            if let Some(ttl) = sprite.lifetime.as_mut() {
                *ttl -= dt;
                if *ttl <= 0.0 {
                    sprite.finished = true;
                }
            }
            // movement
            let (vx, vy) = sprite.velocity;
            let (x, y) = sprite.position;
            let new_x = (x as f32 + vx * dt).round().max(0.0) as u32;
            let new_y = (y as f32 + vy * dt).round().max(0.0) as u32;
            sprite.position = (new_x, new_y);

            if sprite.inanimate || sprite.finished {
                continue;
            }

            // animation
            sprite.time_accumulator += dt;
            if let Some(frame_time) = sprite.frame_time {
                if sprite.time_accumulator >= frame_time {
                    sprite.time_accumulator -= frame_time;

                    if sprite.current_frame + 1 >= sprite.total_frames {
                        if sprite.play_once {
                            sprite.finished = true;
                        } else {
                            sprite.current_frame = 0;
                        }
                    } else {
                        sprite.current_frame += 1;
                    }
                }
            }
        }
        self.active.retain(|sprite| !(sprite.lifetime.is_some() && sprite.finished));

    }

    // provides vector of drawable objects to render pipeline
    pub fn get_drawables(&self) -> Vec<(Sprite, String)> {
        let mut sorted_sprites: Vec<&AnimatedSprite> = self.active.iter().collect();
        sorted_sprites.sort_by_key(|sprite| sprite.strata);

        let mut drawables = Vec::new();

        for sprite_data in sorted_sprites {
            if let Some(texture) = self.textures.get(&sprite_data.texture_id) {
                let mut sprite = Sprite::with_texture(texture);

                let frame_x = (sprite_data.current_frame * sprite_data.frame_width) as i32;
                let frame_y = 0;

                sprite.set_texture_rect(IntRect::new(
                    frame_x,
                    frame_y,
                    sprite_data.frame_width as i32,
                    sprite_data.frame_height as i32,
                ));

                sprite.set_position((
                    sprite_data.position.0 as f32,
                    sprite_data.position.1 as f32,
                ));

                if let (Some(dw), Some(dh)) = (
                    sprite_data.desired_width,
                    sprite_data.desired_height,
                ) {
                    let scale_x = dw as f32 / sprite_data.frame_width as f32;
                    let scale_y = dh as f32 / sprite_data.frame_height as f32;
                    sprite.set_scale((scale_x, scale_y));
                }

                drawables.push((sprite, sprite_data.texture_id.clone()));
            }
        }
        drawables
    }

    pub fn get_drawable(&self, aspr: &AnimatedSprite) -> Option<Sprite> {
        let texture = self.textures.get(&aspr.texture_id)?;

        let mut sprite = Sprite::with_texture(texture);

        let frame_x = (aspr.current_frame * aspr.frame_width) as i32;
        let frame_y = 0;

        sprite.set_texture_rect(IntRect::new(
            frame_x,
            frame_y,
            aspr.frame_width as i32,
            aspr.frame_height as i32,
        ));

        sprite.set_position((aspr.position.0 as f32, aspr.position.1 as f32));

        if let (Some(dw), Some(dh)) = (aspr.desired_width, aspr.desired_height) {
            let scale_x = dw as f32 / aspr.frame_width as f32;
            let scale_y = dh as f32 / aspr.frame_height as f32;
            sprite.set_scale((scale_x, scale_y));
        }

        Some(sprite)
    }

    pub fn load_textures(&mut self, folder_path: &str) {
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
