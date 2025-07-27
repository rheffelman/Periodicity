use sfml::window::{Event};
use crate::{game::Game, properties::ClickAction};

pub enum InputSlot {
    MouseX = 0,
    MouseY = 1,
    LMBCurr = 2,
    LMBPrev = 3,
    Count
}

use InputSlot::*;
impl InputSlot {
    pub const fn count() -> usize {
        InputSlot::Count as usize
    }
}

impl Game {

    pub fn user_input_main_entry(&mut self) {
        self.cache_user_input();
        self.set_hovered_flags();
        self.dispatch_input_handling();

        self.user_input_cache[LMBPrev as usize] =
            self.user_input_cache[LMBCurr as usize];
    }

    fn cache_user_input(&mut self) {
        while let Some(event) = self.window.poll_event() {
            match event {
                Event::Closed => {
                    self.window.close();
                }
                Event::MouseMoved { x, y } => {
                    self.user_input_cache[InputSlot::MouseX as usize] = x as u32;
                    self.user_input_cache[InputSlot::MouseY as usize] = y as u32;
                }
                Event::MouseButtonPressed { button, .. } => {
                    if button == sfml::window::mouse::Button::Left {
                        self.user_input_cache[LMBCurr as usize] = 1;
                    }
                }
                Event::MouseButtonReleased { button, .. } => {
                    if button == sfml::window::mouse::Button::Left {
                        self.user_input_cache[LMBCurr as usize] = 0;
                    }
                }
                _ => {}
            }
        }

    }

    fn dispatch_input_handling(&mut self) {
        if self.user_input_cache[LMBCurr as usize] == 1 && self.user_input_cache[LMBPrev as usize] != 1 {
            self.lmb_pressed();
        }
        else if self.user_input_cache[LMBCurr as usize] == 0 && self.user_input_cache[LMBPrev as usize] == 1 {
            self.lmb_released();
        }
    }

    fn set_hovered_flags(&mut self) {
        let mx = self.user_input_cache[MouseX as usize];
        let my = self.user_input_cache[MouseY as usize];

        for rects in self.em.rectangles.values_mut() {
            for rect in rects.iter_mut() {
                if rect.hovered.is_some() {
                    rect.hovered = None; // purge previous frame
                }

                let within_x = mx >= rect.x && mx <= rect.x + rect.width;
                let within_y = my >= rect.y && my <= rect.y + rect.height;

                if within_x && within_y {
                    rect.hovered = Some(true);
                }
            }
        }
    }

    fn lmb_pressed(&mut self) {
        let mx = self.user_input_cache[MouseX as usize];
        let my = self.user_input_cache[MouseY as usize];
        let buttons = self.em.get_all_buttons();

        let mut clicked_eids = vec![];
        for eid in buttons {
            if let Some(rect) = self.em.get_button_rect_non_mut(eid) {
                let within_x = mx >= rect.x && mx <= rect.x + rect.width;
                let within_y = my >= rect.y && my <= rect.y + rect.height;
                if within_x && within_y {
                    clicked_eids.push(eid);
                }
            }
        }

        for eid in clicked_eids {
            if let Some(cb) = self.em.get_pclickable_non_mut(eid) {
                self.branch_from_click(cb.action.clone());
            }
        }
    }

    fn lmb_released(&mut self) {
        let button_ids = self.em.get_all_buttons();

        for eid in button_ids {
            if let Some(rects) = self.em.get_prects_mut(eid) {
                for rect in rects.iter_mut() {
                    if let Some(_) = rect.pressed {
                        rect.pressed = Some(false);
                    }
                }
            }
        }
    }
}

