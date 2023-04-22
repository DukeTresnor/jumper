// game mod.rs
// create Game plugin
// define SimulationState states -- running and paused
// add systems

mod systems;
mod components;
pub mod resources;
mod player;

use bevy::prelude::*;
//use bevy_rapier2d::prelude::*;

use player::PlayerPlugin;

// use crate::etc. for when trying to access files at a higher level
use crate::AppState;
use crate::events::GameOver;


// access everything in game/systems.rs
use systems::*;


// Constants
pub const GRAVITY: f32 = 98.1; // meters / second^2
pub const TILE_SIZE: f32 = 18.0;

pub struct  GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<SimulationState>()
            .add_event::<GameOver>()
            // pause the simulation once you enter the game state of AppState
            // adds the pause_simulation system on the "OnEnter an AppState Game" schedule
            .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
            .add_system(spawn_floor.in_schedule(OnEnter(AppState::Game)))
            // Plugins to add when inside AppState::Game
            // Player plugin
            .add_plugin(PlayerPlugin)
            .add_systems(
                (
                    apply_gravity,
                )
            
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running)),
            )
            // add the toggle system if you're in the AppState::Game state
            //   use run_if() and send in_state(AppState::Game) into it
            .add_system(toggle_simulation_state.run_if(in_state(AppState::Game)))
            .add_system(despawn_floor.in_schedule(OnExit(AppState::Game)))
            // Add resume_simulation system to the OnExit schedule
            // When you exit the game state, set simulation state to running (ie the default state)
            .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)))
            ;
    }
}

// Declaring states for Game plugin -- funning, paused
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}