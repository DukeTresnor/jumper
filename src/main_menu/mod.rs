// main_menu mod.rs
use bevy::prelude::*;
use systems::*;

mod systems;


pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(main_menu);
    }
}