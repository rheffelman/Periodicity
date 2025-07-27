use std::collections::HashMap;
use sfml::graphics::{Sprite, IntRect};

#[derive(Debug, Clone)]
pub struct Animation {
    pub name: String,
    pub frame_size: (i32, i32), // (width, height)
    pub frame_count: usize,
    pub frame_duration: f32, // seconds per frame
    pub looping: bool,
}

#[derive(Debug, Clone)]
pub struct AnimationState {
    pub current_frame: usize,
    pub timer: f32,
    pub frame_duration: f32,
    pub is_finished: bool,
}

impl AnimationState {
    pub fn new() -> Self {
        Self {
            current_frame: 0,
            timer: 0.0,
            frame_duration: 0.1,
            is_finished: false,
        }
    }

    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.timer = 0.0;
        self.is_finished = false;
    }
}
#[derive(Debug)]
pub struct Animator {
    animations: HashMap<String, Animation>,
    states: HashMap<String, AnimationState>,
}

impl Animator {
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            states: HashMap::new(),
        }
    }

    pub fn add_animation(&mut self, anim: Animation) {
        self.states.insert(anim.name.clone(), AnimationState::new());
        self.animations.insert(anim.name.clone(), anim);
    }

    pub fn update(&mut self, delta_time: f32) {
        for (name, state) in self.states.iter_mut() {
            if let Some(anim) = self.animations.get(name) {
                if state.is_finished && !anim.looping {
                    continue;
                }

                state.timer += delta_time;
                while state.timer >= state.frame_duration {
                    state.timer -= state.frame_duration;
                    state.current_frame += 1;

                    if state.current_frame >= anim.frame_count {
                        if anim.looping {
                            state.current_frame = 0;
                        } else {
                            state.current_frame = anim.frame_count - 1;
                            state.is_finished = true;
                            break;
                        }
                    }
                }
            }
        }
    }

    pub fn apply_to_sprite(&self, anim_name: &str, sprite: &mut Sprite) {
        if let Some(anim) = self.animations.get(anim_name) {
            if let Some(state) = self.states.get(anim_name) {
                let frame_x = (state.current_frame as i32 * anim.frame_size.0) as i32;
                let rect = IntRect::new(
                    frame_x,
                    0,
                    anim.frame_size.0,
                    anim.frame_size.1,
                );
                sprite.set_texture_rect(rect);
            }
        }
    }

    pub fn set_custom_frame_duration(&mut self, anim_name: &str, frame_duration: f32) {
        if let Some(state) = self.states.get_mut(anim_name) {
            state.frame_duration = frame_duration;
        }
    }

    pub fn reset(&mut self, anim_name: &str) {
        if let Some(state) = self.states.get_mut(anim_name) {
            state.reset();
        }
    }

    pub fn is_finished(&self, anim_name: &str) -> bool {
        self.states.get(anim_name).map_or(false, |s| s.is_finished)
    }
}
