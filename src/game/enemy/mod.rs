// game/enemy/mod.rs

use bevy::prelude::*;

pub mod components;
mod systems;



use systems::*;

use crate::AppState;
use crate::game::SimulationState;

pub struct EnemyPlugin;

impl  Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        //app
            
    }
}