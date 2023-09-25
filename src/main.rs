use bevy::prelude::*;
//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;


mod main_menu;
mod game;
mod systems;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))

        .add_state::<AppState>()
        // Plugins for framerate in the console
        //.add_plugins(LogDiagnosticsPlugin::default())
        //.add_plugins(FrameTimeDiagnosticsPlugin::default())
        // Plugins for framerate in the console
        // add plugin for Main Menu
        .add_plugins(MainMenuPlugin)
        // add plugin for Game
        .add_plugins(GamePlugin)
        // add system for spawning camera
        .add_systems(Startup, spawn_camera)
        // add transition systems for changing states:
        .add_systems(Update, transition_to_game_state)
        //    add transition to main menu
        .add_systems(Update, transition_to_main_menu_state)
        // add system for exiting the game
        .add_systems(Update, exit_game)
        // add system for handling the game over state
        .run();
}




// Declaration for MainMenu and Game states
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
}