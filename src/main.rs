use bevy::prelude::*;
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;


mod main_menu;
mod game;
mod systems;
pub mod events;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        // add plugin for Main Menu
        .add_plugin(MainMenuPlugin)
        // add plugin for Game
        .add_plugin(GamePlugin)
        // add system for spawning camera
        .add_startup_system(spawn_camera)
        // add transition systems for changing states:
        //    add transition to game state
        .add_system(transition_to_game_state)
        //    add transition to main menu
        .add_system(transition_to_main_menu_state)
        // add system for exiting the game
        .add_system(exit_game)
        // add system for handling the game over state
        .add_system(handle_game_over_event)
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