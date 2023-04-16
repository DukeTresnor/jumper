use bevy::prelude::*;



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}

// Declaration for MainMenu, Game, and GameOver states
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    MainMenu,
    Game,
    GameOver,
}