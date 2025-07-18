use sfml::cpp::FBox;
use sfml::graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable};
use sfml::window::{Event, Style};
use std::fs;
use serde::{Serialize, Deserialize};

#[link(name = "Advapi32")]

unsafe extern "system" {}

pub static BASE: Color    = Color::rgba(36, 41, 46, 255);
pub static LIGHTER: Color = Color::rgba(43, 49, 55, 255);
pub static REGIONS: Color  = Color::rgba(29, 33, 37, 255);
pub static BUTTON: Color  = Color::rgba(19, 81, 150, 255);
pub static BORDER: Color  = Color::rgba(24, 26, 28, 255);

const BUTTONS_FP: &str = "./src/gamedata/buttons.json";
struct Button<'s> {
    shape: RectangleShape<'s>,
    width: u32,
    height: u32
}

#[derive(Serialize, Deserialize)]
struct ButtonConfig {
    width: u32,
    height: u32,
    pos_x: u32,
    pos_y: u32,
}

impl<'s> Button<'s> {
    fn new(width: u32, height: u32, pos_x: u32, pos_y: u32) -> Button<'s> {
        let mut rect = RectangleShape::new();
        rect.set_size((width as f32, height as f32));
        rect.set_fill_color(BUTTON);
        rect.set_outline_color(BORDER);
        rect.set_outline_thickness(2.0);
        rect.set_position((pos_x as f32, pos_y as f32));
        Button { shape: rect, width, height }
    }

    fn to_config(&self) -> ButtonConfig {
        let pos = self.shape.position();
        ButtonConfig {
            width: self.width,
            height: self.height,
            pos_x: pos.x as u32,
            pos_y: pos.y as u32,
        }
    }

    fn from_config(cfg: &ButtonConfig) -> Button<'s> {
        Button::new(cfg.width, cfg.height, cfg.pos_x, cfg.pos_y)
    }
}

struct Region<'s> {
    shape: RectangleShape<'s>,
    width: u32,
    height: u32
}

#[derive(Serialize, Deserialize)]
struct RegionConfig {
    width: u32,
    height: u32,
    pos_x: u32,
    pos_y: u32,
}


impl<'s> Region<'s> {
    fn new(width: u32, height: u32, pos_x: u32, pos_y: u32) -> Region<'s> {
        let mut rect = RectangleShape::new();
        rect.set_size((width as f32, height as f32));
        rect.set_fill_color(BUTTON);
        rect.set_outline_color(BORDER);
        rect.set_outline_thickness(2.0);
        rect.set_position((pos_x as f32, pos_y as f32));
        Region { shape: rect, width, height }
    }

    fn to_config(&self) -> RegionConfig {
        let pos = self.shape.position();
        RegionConfig {
            width: self.width,
            height: self.height,
            pos_x: pos.x as u32,
            pos_y: pos.y as u32,
        }
    }

    fn from_config(cfg: &RegionConfig) -> Region<'s> {
        Region::new(cfg.width, cfg.height, cfg.pos_x, cfg.pos_y)
    }
}
struct Game<'s> {
    window: FBox<RenderWindow>,
    buttons: Vec<Button<'s>>
}
impl<'s> Game<'s> {

    fn new() -> Game<'s> {
        let mut window = RenderWindow::new(
            (1920, 1080),
            "RPGame",
            Style::CLOSE,
            &Default::default(),
        )
        .expect("Failed to create SFML RenderWindow");
        window.set_vertical_sync_enabled(true);

        Game { window, buttons: Vec::new() }
    }

    fn save_buttons(&self, button_path: &str) {
        let btns: Vec<ButtonConfig> = self.buttons.iter().map(|b| b.to_config()).collect();
        let json = serde_json::to_string_pretty(&btns).unwrap();
        fs::write(button_path, json).unwrap();
    }

    fn load_buttons(&mut self, button_path: &str) {
        let data = fs::read_to_string(button_path).unwrap_or_else(|_| "[]".to_string());
        let btns: Vec<ButtonConfig> = serde_json::from_str(&data).unwrap();
        self.buttons = btns.iter().map(|cfg| Button::from_config(cfg)).collect();
    }


    fn init(&mut self) {
        self.load_buttons(BUTTONS_FP);
    }

    fn run(&mut self) {
        while self.window.is_open() {
            self.user_input();
            self.render();
        }
    }

    fn user_input(&mut self) {
        while let Some(event) = self.window.poll_event() {
            if event == Event::Closed {
                self.window.close();
            }
        }
    }

    fn render(&mut self) {
        self.window.clear(BASE);

        // render GUI
        for b in &self.buttons {
            self.window.draw(&b.shape);
        }

        self.window.display();
    }
}

fn main() {
    let mut g = Game::new();
    g.init();
    g.run();
}

