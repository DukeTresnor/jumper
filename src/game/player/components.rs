// game/player/components.rs

// why is this here?
//use std::thread::local_impl::Key;

use bevy::prelude::*;

// Define Player component
#[derive(Component)]
pub struct Player {}


#[derive(Component)]
pub struct JumpVelocity {
    pub horizontal_velocity: f32,
    pub vertical_velocity: f32,
}


// action vector has a keycode and a f32, the f32 is meant to hold time values
#[derive(Component)]
pub struct ActionStateVector {

    pub action_vector: Vec<(KeyCode, f32)>,
}

#[derive(Component)]
pub struct  NegativeEdgeStateVector {
    pub negative_edge_vector: Vec<(KeyCode, f32)>,
}


#[derive(Component)]
pub struct PlayerInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub light: bool,
    pub medium: bool,
    pub heavy: bool,
    pub special: bool,
}


#[derive(Component)]
pub struct InputBinding {
    pub up_bind: KeyCode,
    pub down_bind: KeyCode,
    pub left_bind: KeyCode,
    pub right_bind: KeyCode,
    pub light_bind: KeyCode,
    pub medium_bind: KeyCode,
    pub heavy_bind: KeyCode,
    pub special_bind: KeyCode,
}

#[derive(Component)]
pub struct MovementState {
    pub is_grounded: bool,
}

#[derive(Component)]
pub struct AttackState {
    pub is_attacking: bool,
}

#[derive(Component)]
pub struct PlayerNumber {
    pub player_number: i32,
}