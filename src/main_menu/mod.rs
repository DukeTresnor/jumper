// main_menu mod.rs
mod components;
mod styles;
mod systems;

use bevy::prelude::*;

use systems::layout::*;
use systems::interactions::*;

use crate::AppState;





pub struct MainMenuPlugin;

// Implements the Plugin trait for MainMenuPlugin struct, which requires the build method to be included
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
        .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)
        ;
    }
}


