mod entities;
mod properties;
mod game;
mod user_input;
mod construct_window;
mod render_pipeline;
mod button_definitions;
mod helpers;
mod g_entities;
mod g_properties;
mod construct_game;
mod update_game;
mod animation;
mod systems;

#[link(name = "Advapi32")]
unsafe extern "system" {}

fn main() {
    let mut g = game::Game::new();
    g.init_main_entry(); // branch to construct_window.rs
    g.run(); // branch to game.rs
}