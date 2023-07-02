// game/player/components.rs

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
