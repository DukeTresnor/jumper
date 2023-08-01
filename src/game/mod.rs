// game mod.rs
// create Game plugin
// define SimulationState states -- running and paused
// add systems

mod systems;
mod components;
pub mod resources;
mod player;

mod enemy;

use bevy::prelude::*;

use player::PlayerPlugin;
use enemy::EnemyPlugin;

// use crate::etc. for when trying to access files at a higher level
use crate::AppState;
//use crate::events::GameOver;

// access everything in game/systems.rs
use systems::*;

use self::resources::InputBufferTimer;


// Constants
pub const GRAVITY: f32 = 98.1; // meters / second^2
pub const TILE_SIZE: f32 = 18.0;

pub struct  GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<InputBufferTimer>()
            .add_state::<SimulationState>()
            //.add_event::<GameOver>()
            // pause the simulation once you enter the game state of AppState
            // adds the pause_simulation system on the "OnEnter an AppState Game" schedule                  app.add_systems(OnEnter(AppState::Menu), enter_menu)
            .add_systems(OnEnter(AppState::Game), pause_simulation)
            .add_systems(OnEnter(AppState::Game), spawn_floor)
            // Plugins to add when inside AppState::Game
            // Player plugin
            .add_plugins(PlayerPlugin)
            .add_plugins(EnemyPlugin)
            .add_systems(
                Update,
                (
                    animate_sprite,
                    apply_gravity_and_velocity,
                )
            
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running)),
            )
            // add the toggle system if you're in the AppState::Game state
            //   use run_if() and send in_state(AppState::Game) into it
            .add_systems(Update, toggle_simulation_state.run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), despawn_floor)
            // Add resume_simulation system to the OnExit schedule
            // When you exit the game state, set simulation state to running (ie the default state)
            .add_systems(OnExit(AppState::Game), resume_simulation)
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