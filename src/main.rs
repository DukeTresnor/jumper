use bevy::prelude::*;


mod main_menu;
mod game;
mod systems;
pub mod events;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        // add plugin for Game
        // add plugin for Main Menu
        // add system for spawning camera
        // add transition systems for changing states:
        //    add transition to game state
        //    add transition to main menu
        // add system for exiting the game
        // add system for handling the game over state
        .run();
}

// Declaration for MainMenu, Game, and GameOver states
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}