// game/player/mod.rs

use bevy::prelude::*;

pub mod components;
mod systems;



use systems::*;

use crate::AppState;
use crate::game::SimulationState;


pub const PLAYER_SPEED_VERTICAL: f32 = 400.0;
pub const PLAYER_SPEED_HORIZONTAL: f32 = 100.0;
pub const PLAYER_SIZE: f32 = 64.0; // This is the player sprite size, not necessarily the size of each frame in the spritesheet

pub const SPECIAL_MOVE_BUFFER_TIME: f32 = 0.5;
pub const DIRECTION_JUMP_BUFFER_TIME: f32 = 0.5;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_state::<PlayerState>()
            //.add_state::<AttackState>()
            .add_systems(OnEnter(AppState::Game), spawn_player)
            .add_systems(
                Update,
                (
                    populate_player_action_vector,
                    confine_player_movement,
                    temp_player_up_movement,
                    ground_check,
                    player_reset_to_neutral,
                    testing_new_input_system,
                    player_ground_attack,
                    player_movement,
                )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running))
            )
            /*
            .add_systems(
                (
                    //player_jump,
                    player_ground_attack,
                    player_movement,
                )
            
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running))
                .in_set(OnUpdate(PlayerState::Grounded))
            )
             */
            //.add_system(ground_check.run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), despawn_player)
            ;
    }
}


// Adding player state enum
// Add player movement systems

/* 
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum PlayerState {
    #[default]
    Grounded,
    Air,
}


// was grounded state
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AttackState {
    #[default]
    Neutral,
    Attack,
}
*/


/* 
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AirState {
    #[default]
    Neutral,
    Attack,
}
*/