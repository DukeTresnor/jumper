// game mod.rs
// create Game plugin
// define SimulationState states -- running and paused
// add systems

use bevy::prelude::*;



// use crate::etc. for when trying to access files at a higher level
use crate::AppState;
use crate::events::GameOver;


// access everything in game/systems.rs
mod systems::*;

pub struct  GamePlugin;



// Declaring states for Game plugin -- funning, paused
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}