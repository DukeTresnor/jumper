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

#[derive(Component)]
pub struct ActionStateVector {
    //pub action_vector: Vec<KeyCode>,
    pub action_vector: Vec<(KeyCode, ActionTimerValue)>,
}

#[derive(Component)]
pub struct ActionTimerValue {
    pub action_timer: f32,
}