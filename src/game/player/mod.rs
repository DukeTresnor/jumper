// game/player/mod.rs

use bevy::prelude::*;

pub mod components;
mod systems;



use systems::*;

use crate::AppState;
use crate::game::SimulationState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)))
            ;
    }
}


// Adding player state enum
// Add player movement systems
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum PlayerState {
    #[default]
    Grounded,
    Air,
}